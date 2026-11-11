//! Local persistence of completed telemetry sessions.
//! Session records are stored as JSON files under
//! <data_dir>/f1-telemetry/sessions/ (mirrors the existing settings.json pattern).

use crate::telemetry::broadcaster::{LapHistoryEntry, ParticipantInfo, SessionState};
use crate::telemetry::packets::{TelemetryEvent, CarTelemetryData, CarStatusData, MAX_CARS};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

/// Directory used for all persisted session data.
pub fn sessions_dir() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("f1-telemetry");
    path.push("sessions");
    let _ = fs::create_dir_all(&path);
    path
}

/// Lightweight item used when listing past sessions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionListItem {
    pub id: String,
    pub session_uid: u64,
    pub track_name: String,
    pub session_type: String,
    pub started_at: i64, // unix ms
    pub duration_ms: u16,
    pub best_lap_time: u32,
    pub total_laps: u8,
    pub weather: String,
}

/// Full record of a single session, persisted to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: String,
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
    pub started_at: i64, // unix ms
    pub participants: Vec<ParticipantInfo>,
    pub lap_history: Vec<LapHistoryEntry>,
    pub best_lap_time: u32,
    pub last_lap_time: u32,
    pub sector_times: [u16; 3],
    pub best_sector_times: [u16; 3],
}

impl SessionRecord {
    /// Build a `SessionRecord` from the current live `SessionState`.
    pub fn from_state(state: &SessionState, id: String, started_at: i64) -> Self {
        Self {
            id,
            session_uid: state.session_uid,
            track_name: state.track_name.clone(),
            session_type: state.session_type.clone(),
            session_time_left: state.session_time_left,
            session_duration: state.session_duration,
            weather: state.weather.clone(),
            track_temp: state.track_temp,
            air_temp: state.air_temp,
            total_laps: state.total_laps,
            player_car_index: state.player_car_index,
            active_driver_index: state.active_driver_index,
            num_active_cars: state.num_active_cars,
            started_at,
            participants: state.participants.clone(),
            lap_history: state.lap_history.iter().cloned().collect(),
            best_lap_time: state.best_lap_time,
            last_lap_time: state.last_lap_time,
            sector_times: state.sector_times,
            best_sector_times: state.best_sector_times,
        }
    }

    /// Lightweight listing view of this record.
    pub fn list_item(&self) -> SessionListItem {
        SessionListItem {
            id: self.id.clone(),
            session_uid: self.session_uid,
            track_name: self.track_name.clone(),
            session_type: self.session_type.clone(),
            started_at: self.started_at,
            duration_ms: self.session_duration,
            best_lap_time: self.best_lap_time,
            total_laps: self.total_laps,
            weather: self.weather.clone(),
        }
    }
}

/// Path to the JSON file for a given session id.
fn session_path(id: &str) -> PathBuf {
    sessions_dir().join(format!("{}.json", id))
}

/// Generate a stable file id for a session.
pub fn make_session_id(session_uid: u64, started_at: i64) -> String {
    format!("{}_{}", session_uid, started_at)
}

/// Persist a session record to disk.
pub fn save_session(record: &SessionRecord) -> Result<(), String> {
    let path = session_path(&record.id);
    let json = serde_json::to_string_pretty(record).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// List all persisted sessions, newest first.
pub fn list_sessions() -> Result<Vec<SessionListItem>, String> {
    let dir = sessions_dir();
    let mut items: Vec<SessionListItem> = Vec::new();

    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let record: SessionRecord = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        items.push(record.list_item());
    }

    items.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    Ok(items)
}

/// Load a single session record by id.
pub fn get_session(id: &str) -> Result<SessionRecord, String> {
    let path = session_path(id);
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

/// Delete a persisted session by id.
pub fn delete_session(id: &str) -> Result<(), String> {
    let path = session_path(id);
    fs::remove_file(&path).map_err(|e| e.to_string())
}

/// Persist the current live session when a session boundary is detected.
pub fn persist_current_session(state: &SessionState, started_at: i64) -> Result<String, String> {
    let id = make_session_id(state.session_uid, started_at);
    let record = SessionRecord::from_state(state, id.clone(), started_at);
    save_session(&record)?;
    Ok(id)
}
