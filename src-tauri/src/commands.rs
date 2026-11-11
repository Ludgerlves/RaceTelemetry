use tauri::AppHandle;
use crate::telemetry::udp_listener;
use crate::telemetry::broadcaster::get_session_manager;
use crate::telemetry::ac_shared_memory;
use crate::telemetry::persistence;
use std::sync::Mutex;
use std::time::Duration;
use tracing::{info, error};
use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::net::UdpSocket;
use tokio::time::timeout;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "udpPort")]
    udp_port: u16,
    #[serde(rename = "units")]
    units: String,
    #[serde(rename = "theme")]
    theme: String,
    #[serde(rename = "autoConnect")]
    auto_connect: bool,
    #[serde(rename = "logLevel")]
    log_level: String,
    // Machine IP Configuration
    #[serde(rename = "sourceIpOverride")]
    source_ip_override: String,
    #[serde(rename = "targetIp")]
    target_ip: String,
    #[serde(rename = "targetPort")]
    target_port: u16,
    #[serde(rename = "enableForwarding")]
    enable_forwarding: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            udp_port: 20777,
            units: "metric".to_string(),
            theme: "f1-dark".to_string(),
            auto_connect: true,
            log_level: "info".to_string(),
            source_ip_override: "".to_string(),
            target_ip: "".to_string(),
            target_port: 20777,
            enable_forwarding: false,
        }
    }
}

fn get_settings_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("f1-telemetry");
    fs::create_dir_all(&path).ok();
    path.push("settings.json");
    path
}

/// Global state for network info
struct NetworkState {
    detected_source_ip: Mutex<Option<(String, u16)>>,
}

impl NetworkState {
    fn new() -> Self {
        Self {
            detected_source_ip: Mutex::new(None),
        }
    }
}

static NETWORK_STATE: std::sync::OnceLock<NetworkState> = std::sync::OnceLock::new();

fn get_network_state() -> &'static NetworkState {
    NETWORK_STATE.get_or_init(|| NetworkState::new())
}

/// Set the detected source IP (called from UDP listener)
pub fn set_detected_source_ip(ip: String, port: u16) {
    let state = get_network_state();
    *state.detected_source_ip.lock().unwrap() = Some((ip, port));
}

/// Command to start the UDP telemetry listener
#[tauri::command]
pub async fn start_telemetry(app_handle: AppHandle) -> Result<String, String> {
    crate::telemetry::broadcaster::init_session_manager(app_handle.clone());

    match udp_listener::start_udp_listener(app_handle).await {
        Ok(()) => {
            info!("Telemetry listener started successfully.");
            Ok("Telemetry service running.".into())
        }
        Err(e) => {
            error!("Failed to start telemetry service: {}", e);
            Err(format!("Failed to start telemetry service: {}", e))
        }
    }
}

/// Command to stop the UDP telemetry listener
#[tauri::command]
pub async fn stop_telemetry() -> Result<String, String> {
    let stopped = udp_listener::stop_listener();
    if stopped {
        info!("Telemetry listener stopped.");
        Ok("Telemetry service stopped.".into())
    } else {
        Ok("Telemetry service was not running.".into())
    }
}

/// Command to get current session info
#[tauri::command]
pub fn get_session_info() -> String {
    let manager = get_session_manager();
    let state = manager.get_state();
    serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_string())
}

/// Command to get available drivers (from participants data)
#[tauri::command]
pub fn get_available_drivers() -> Vec<String> {
    let manager = get_session_manager();
    let state = manager.get_state();
    state.participants.iter().map(|p| format!("{}: {}", p.name, p.car_index)).collect()
}

/// Command to set the active driver (player car index)
#[tauri::command]
pub fn set_active_driver(player_index: u8) -> Result<String, String> {
    let manager = get_session_manager();
    manager.set_active_driver(player_index);
    info!("Active driver set to index {}", player_index);
    Ok(format!("Active driver set to index {}", player_index))
}

/// Command to export session data
#[tauri::command]
pub fn export_session(format: String) -> Result<String, String> {
    let manager = get_session_manager();
    let state = manager.get_state();

    let data = match format.as_str() {
        "json" => serde_json::to_string_pretty(&state).map_err(|e| e.to_string()),
        "csv" => {
            // Simple CSV export of lap history
            let mut csv = String::from("Lap,LapTime,S1,S2,S3,Valid\n");
            for lap in &state.lap_history {
                csv.push_str(&format!("{},{},{},{},{},{}\n",
                    lap.lap_number,
                    lap.lap_time_ms,
                    lap.sector1_ms,
                    lap.sector2_ms,
                    lap.sector3_ms,
                    lap.is_valid
                ));
            }
            Ok(csv)
        },
        _ => Err("Invalid format".to_string()),
    };

    match data {
        Ok(d) => {
            let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
            path.push("f1-telemetry");
            fs::create_dir_all(&path).ok();
            let filename = format!("session_export_{}.{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"), format);
            path.push(filename);
            fs::write(&path, d).map_err(|e| e.to_string())?;
            Ok(format!("Exported to {:?}", path))
        }
        Err(e) => Err(e.to_string())
    }
}

/// Command to clear session data
#[tauri::command]
pub fn clear_session_data() -> Result<String, String> {
    let manager = get_session_manager();
    manager.clear_session();
    Ok("Session data cleared.".into())
}

/// Command to list all persisted sessions (newest first)
#[tauri::command]
pub fn list_sessions() -> Result<Vec<persistence::SessionListItem>, String> {
    persistence::list_sessions()
}

/// Command to get a single persisted session by id
#[tauri::command]
pub fn get_session(id: String) -> Result<persistence::SessionRecord, String> {
    persistence::get_session(&id)
}

/// Command to delete a persisted session by id
#[tauri::command]
pub fn delete_session(id: String) -> Result<String, String> {
    persistence::delete_session(&id).map(|_| format!("Session {} deleted", id))
}

/// Command to save the current live session to disk
#[tauri::command]
pub fn save_current_session() -> Result<String, String> {
    let manager = get_session_manager();
    let state = manager.get_state();
    let started_at = chrono::Utc::now().timestamp_millis();
    persistence::persist_current_session(&state, started_at)
}

/// Get settings
#[tauri::command]
pub fn get_settings() -> Settings {
    let path = get_settings_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Settings::default()
    }
}

/// Save settings
#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<SaveSettingsResult, String> {
    let path = get_settings_path();
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(SaveSettingsResult {
        success: true,
        message: "Settings saved successfully".to_string(),
    })
}

/// Result for save_settings command
#[derive(Serialize)]
pub struct SaveSettingsResult {
    success: bool,
    message: String,
}

/// Result for test_connection command
#[derive(Serialize)]
pub struct ConnectionTestResult {
    success: bool,
    message: String,
    detected_ip: Option<String>,
    detected_port: Option<u16>,
    packet_count: u64,
}

/// Test connection by listening for UDP packets from PS5 for a short duration
#[tauri::command]
pub async fn test_connection(port: u16) -> Result<ConnectionTestResult, String> {
    info!("Testing connection on port {}", port);

    // Try to bind to the port first to check if it's available
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
        Ok(s) => s,
        Err(e) => {
            return Ok(ConnectionTestResult {
                success: false,
                message: format!("Failed to bind to port {}: {}", port, e),
                detected_ip: None,
                detected_port: None,
                packet_count: 0,
            });
        }
    };

    info!("Test connection: UDP socket bound to port {}", port);

    let mut buf = [0u8; 2048];
    let mut packet_count: u64 = 0;
    let mut detected_ip: Option<String> = None;
    let mut detected_port: Option<u16> = None;
    let start_time = std::time::Instant::now();
    let test_duration = Duration::from_secs(5); // Listen for 5 seconds

    // Set a read timeout so we can check elapsed time
    let mut last_packet_time = std::time::Instant::now();

    loop {
        // Check if test duration has elapsed
        if start_time.elapsed() >= test_duration {
            break;
        }

        // Use a short timeout for recv_from so we can check elapsed time periodically
        let remaining = test_duration.saturating_sub(start_time.elapsed());
        let recv_future = socket.recv_from(&mut buf);

        match timeout(remaining.min(Duration::from_millis(500)), recv_future).await {
            Ok(Ok((len, addr))) => {
                packet_count += 1;
                last_packet_time = std::time::Instant::now();

                // Record the source IP of the first packet
                if detected_ip.is_none() {
                    detected_ip = Some(addr.ip().to_string());
                    detected_port = Some(addr.port());
                    info!("Test connection: Detected source {}:{}", detected_ip.as_ref().unwrap(), detected_port.unwrap());
                }

                // If we got packets, we can consider it a success
                // but keep listening for the full duration to get packet count
            }
            Ok(Err(e)) => {
                error!("Test connection: UDP receive error: {}", e);
                break;
            }
            Err(_) => {
                // Timeout - no packet received in this interval, continue loop
                // This allows us to check if overall test duration has elapsed
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs_f32();

    if packet_count > 0 {
        Ok(ConnectionTestResult {
            success: true,
            message: format!("Connected! Received {} packets from {}:{} in {:.1}s",
                packet_count,
                detected_ip.as_ref().unwrap(),
                detected_port.unwrap(),
                elapsed),
            detected_ip,
            detected_port,
            packet_count,
        })
    } else {
        Ok(ConnectionTestResult {
            success: false,
            message: format!("No packets received in {:.1}s. Check PS5 IP settings and ensure game is sending telemetry.", elapsed),
            detected_ip: None,
            detected_port: None,
            packet_count: 0,
        })
    }
}

/// Response for detected source IP
#[derive(Serialize)]
pub struct SourceIpResponse {
    ip: Option<String>,
    port: Option<u16>,
}

/// Command to get the detected source IP (where the F1 game is sending data from)
#[tauri::command]
pub fn get_detected_source_ip() -> SourceIpResponse {
    let state = get_network_state();
    let detected = state.detected_source_ip.lock().unwrap();
    match &*detected {
        Some((ip, port)) => SourceIpResponse {
            ip: Some(ip.clone()),
            port: Some(*port),
        },
        None => SourceIpResponse {
            ip: None,
            port: None,
        },
    }
}

/// Command to get local IP addresses (to configure in F1 game telemetry settings)
#[tauri::command]
pub fn get_local_ips() -> Vec<String> {
    udp_listener::get_local_ips()
}

/// Command to start Assetto Corsa shared memory reader
#[tauri::command]
pub async fn start_shared_memory(app_handle: tauri::AppHandle) -> Result<String, String> {
    match ac_shared_memory::start_shared_memory(app_handle) {
        Ok(()) => {
            info!("Assetto Corsa shared memory reader started.");
            Ok("Assetto Corsa shared memory reader running.".into())
        }
        Err(e) => {
            error!("Failed to start Assetto Corsa shared memory reader: {}", e);
            Err(format!("Failed to start shared memory reader: {}", e))
        }
    }
}

/// Command to stop Assetto Corsa shared memory reader
#[tauri::command]
pub fn stop_shared_memory() -> Result<String, String> {
    let stopped = ac_shared_memory::stop_shared_memory();
    if stopped {
        info!("Assetto Corsa shared memory reader stopped.");
        Ok("Assetto Corsa shared memory reader stopped.".into())
    } else {
        Ok("Assetto Corsa shared memory reader was not running.".into())
    }
}

/// Command to check if Assetto Corsa shared memory reader is running
#[tauri::command]
pub fn is_shared_memory_running() -> bool {
    ac_shared_memory::is_shared_memory_running()
}