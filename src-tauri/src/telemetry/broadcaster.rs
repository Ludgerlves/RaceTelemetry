//! Broadcaster for telemetry events
//! Handles emitting events to the frontend via Tauri

use crate::telemetry::game_detection::GameId;
use crate::telemetry::packets::{TelemetryEvent, CarTelemetryData, CarStatusData, PacketHeader, LapData, MAX_CARS};
use crate::telemetry::persistence;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tracing::{debug, info};
use serde::{Serialize, Deserialize};
use chrono::Utc;

/// Maximum number of lap history entries to keep
const MAX_LAP_HISTORY: usize = 100;

/// Session state shared between backend and frontend
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionState {
    pub session_uid: u64,
    pub track_name: String,
    pub session_type: String,
    pub session_time_left: u16,
    pub session_duration: u16,
    pub weather: String,
    pub track_temp: i8,
    pub air_temp: i8,
    pub total_laps: u8,
    pub player_car_index: u8,
    pub active_driver_index: u8,
    pub num_active_cars: u8,
    pub active_game: Option<GameId>,
    pub session_started_at: Option<i64>,
    pub participants: Vec<ParticipantInfo>,
    pub current_lap: u8,
    pub best_lap_time: u32,
    pub last_lap_time: u32,
    pub sector_times: [u16; 3],
    pub best_sector_times: [u16; 3],
    #[serde(with = "serde_arrays::LapHistoryArray")]
    pub lap_history: VecDeque<LapHistoryEntry>,
    pub is_new_session: bool,
}

/// Serde helpers for large arrays
mod serde_arrays {
    use super::LapHistoryEntry;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::collections::VecDeque;

    pub mod LapHistoryArray {
        use super::*;
        pub fn serialize<S>(deque: &VecDeque<LapHistoryEntry>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.collect_seq(deque.iter())
        }
        pub fn deserialize<'de, D>(deserializer: D) -> Result<VecDeque<LapHistoryEntry>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let vec: Vec<LapHistoryEntry> = Vec::deserialize(deserializer)?;
            Ok(VecDeque::from(vec))
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParticipantInfo {
    pub car_index: u8,
    pub driver_id: u8,
    pub name: String,
    pub team_id: u8,
    pub race_number: u8,
    pub nationality: u8,
    pub ai_controlled: bool,
    pub your_telemetry: bool,
    pub car_color_primary: u8,
    pub car_color_secondary: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LapHistoryEntry {
    pub lap_number: u8,
    pub lap_time_ms: u32,
    pub sector1_ms: u16,
    pub sector2_ms: u16,
    pub sector3_ms: u16,
    pub is_valid: bool,
    pub timestamp: u64,
}

/// Global session state manager
pub struct SessionManager {
    state: Arc<Mutex<SessionState>>,
    app_handle: Arc<Mutex<Option<AppHandle>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(SessionState::default())),
            app_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_app_handle(&self, handle: AppHandle) {
        *self.app_handle.lock().unwrap() = Some(handle);
    }

    pub fn get_state(&self) -> SessionState {
        self.state.lock().unwrap().clone()
    }

    pub fn update_from_session_packet(&self, packet: &crate::telemetry::packets::PacketSessionData) {
        let mut state = self.state.lock().unwrap();

        // Check for new session
        if state.session_uid != packet.m_header.m_session_uid {
            // Persist the previous session before switching (if there was one).
            if state.session_uid != 0 && state.session_started_at.is_some() {
                let previous = state.clone();
                drop(state);
                let _ = persistence::persist_current_session(
                    &previous,
                    previous.session_started_at.unwrap(),
                );
                state = self.state.lock().unwrap();
            }

            state.session_uid = packet.m_header.m_session_uid;
            state.is_new_session = true;
            state.lap_history.clear();
            state.best_lap_time = u32::MAX;
            state.current_lap = 0;
            state.session_started_at = Some(Utc::now().timestamp_millis());
            let session_uid = packet.m_header.m_session_uid;
            info!("New session detected: UID={}", session_uid);
        } else {
            state.is_new_session = false;
        }

        state.track_name = format!("{}", packet.m_track_id);
        state.session_type = format!("{}", packet.m_session_type);
        state.session_time_left = packet.m_session_time_left;
        state.session_duration = packet.m_session_duration;
        state.weather = format!("{:?}", packet.m_weather);
        state.track_temp = packet.m_track_temperature;
        state.air_temp = packet.m_air_temperature;
        state.total_laps = packet.m_total_laps;
        state.player_car_index = packet.m_header.m_player_car_index;
    }

    pub fn update_from_participants_packet(&self, packet: &crate::telemetry::packets::PacketParticipantsData) {
        let mut state = self.state.lock().unwrap();
        state.num_active_cars = packet.m_num_active_cars;
        state.participants = packet.m_participants.iter()
            .take(packet.m_num_active_cars as usize)
            .enumerate()
            .map(|(i, p)| ParticipantInfo {
                car_index: i as u8,
                driver_id: p.m_driver_id,
                name: String::from_utf8_lossy(&p.m_name).trim_end_matches('\0').to_string(),
                team_id: p.m_team_id as u8,
                race_number: p.m_race_number,
                nationality: p.m_nationality,
                ai_controlled: p.m_ai_controlled != 0,
                your_telemetry: p.m_your_telemetry != 0,
                car_color_primary: p.m_car_colour_primary,
                car_color_secondary: p.m_car_colour_secondary,
            })
            .collect();
    }

    pub fn update_from_lap_data(&self, header: &PacketHeader, lap_data: &[LapData; MAX_CARS], player_index: u8) {
        let mut state = self.state.lock().unwrap();

        if let Some(player_data) = lap_data.get(player_index as usize) {
            // Check for new lap
            if player_data.m_current_lap_num > state.current_lap && state.current_lap > 0 {
                // Previous lap completed - add to history
                if player_data.m_last_lap_time_in_ms > 0 && player_data.m_last_lap_time_in_ms < 10_000_000 {
                    let entry = LapHistoryEntry {
                        lap_number: state.current_lap,
                        lap_time_ms: state.last_lap_time,
                        sector1_ms: state.sector_times[0],
                        sector2_ms: state.sector_times[1],
                        sector3_ms: state.sector_times[2],
                        is_valid: player_data.m_current_lap_invalid == 0,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    };
                    state.lap_history.push_back(entry);
                    if state.lap_history.len() > MAX_LAP_HISTORY {
                        state.lap_history.pop_front();
                    }

                    // Update best lap
                    if state.last_lap_time < state.best_lap_time {
                        state.best_lap_time = state.last_lap_time;
                        state.best_sector_times = state.sector_times;
                    }
                }
            }

            state.current_lap = player_data.m_current_lap_num;
            state.last_lap_time = player_data.m_last_lap_time_in_ms;
            state.sector_times = [
                player_data.m_sector1_time_in_ms,
                player_data.m_sector2_time_in_ms,
                player_data.m_sector3_time_in_ms,
            ];
        }
    }

    pub fn update_from_telemetry(&self, header: &PacketHeader, telemetry: &[CarTelemetryData; MAX_CARS], player_index: u8) {
        // Just forward to frontend via event - state is minimal
    }

    pub fn update_from_status(&self, header: &PacketHeader, status: &[CarStatusData; MAX_CARS], player_index: u8) {
        // Just forward to frontend via event - state is minimal
    }

    /// Set the active driver index used by the frontend to select which car's telemetry to display.
    pub fn set_active_driver(&self, index: u8) {
        let mut state = self.state.lock().unwrap();
        state.active_driver_index = index;
        drop(state);
        self.emit_session_state();
    }

    /// Set the currently active game. When the game changes, reset the parser/broadcaster
    /// state (lap history, best lap, current lap, session uid) so telemetry from a different
    /// game is not misinterpreted as a continuation of the previous game's session.
    pub fn set_active_game(&self, game_id: GameId) {
        let mut state = self.state.lock().unwrap();

        if state.active_game != Some(game_id) {
            // Game changed — reset session-scoped state so the new game starts clean.
            if state.active_game.is_some() && state.session_started_at.is_some() {
                let previous = state.clone();
                drop(state);
                let _ = persistence::persist_current_session(
                    &previous,
                    previous.session_started_at.unwrap(),
                );
                state = self.state.lock().unwrap();
            }

            state.active_game = Some(game_id);
            state.session_uid = 0;
            state.is_new_session = true;
            state.lap_history.clear();
            state.best_lap_time = u32::MAX;
            state.current_lap = 0;
            state.session_started_at = Some(Utc::now().timestamp_millis());
            info!("Game changed to {:?}: session state reset", game_id);
        }

        drop(state);
        self.emit_session_state();
    }

    /// Clear all live session state (lap history, best lap, current lap, sectors)
    pub fn clear_session(&self) {
        let mut state = self.state.lock().unwrap();
        state.lap_history.clear();
        state.best_lap_time = u32::MAX;
        state.current_lap = 0;
        state.last_lap_time = 0;
        state.sector_times = [0, 0, 0];
        drop(state);
        self.emit_session_state();
    }

    /// Emit session state to frontend
    pub fn emit_session_state(&self) {
        if let Some(app_handle) = self.app_handle.lock().unwrap().as_ref() {
            let state = self.get_state();
            let _ = app_handle.emit("session-state", &state);
        }
    }
}

/// Default implementation
impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// Global instance
static SESSION_MANAGER: std::sync::OnceLock<SessionManager> = std::sync::OnceLock::new();

pub fn get_session_manager() -> &'static SessionManager {
    SESSION_MANAGER.get_or_init(|| SessionManager::new())
}

pub fn init_session_manager(app_handle: AppHandle) {
    if let Some(manager) = SESSION_MANAGER.get() {
        manager.set_app_handle(app_handle);
    } else {
        let mut manager = SessionManager::new();
        manager.set_app_handle(app_handle);
        let _ = SESSION_MANAGER.set(manager);
    }
}