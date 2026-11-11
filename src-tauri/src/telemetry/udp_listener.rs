//! UDP Listener for F1 25 Telemetry
//! Listens on port 20777 for UDP packets and parses them

use crate::telemetry::game_detection::{GameId, GameRegistry};
use crate::telemetry::packets::*;
use crate::telemetry::parser::{parse_packet, ParseError};
use crate::commands::set_detected_source_ip;
use std::net::SocketAddr;
use std::sync::{OnceLock, Mutex};
use std::time::{Duration, Instant};
use tauri::Emitter;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

/// Global handle to the currently running UDP listener task (if any).
static LISTENER_HANDLE: OnceLock<Mutex<Option<JoinHandle<()>>>> = OnceLock::new();

/// Global game registry for multi-game detection
static GAME_REGISTRY: OnceLock<Mutex<GameRegistry>> = OnceLock::new();

fn get_listener_handle() -> &'static Mutex<Option<JoinHandle<()>>> {
    LISTENER_HANDLE.get_or_init(|| Mutex::new(None))
}

fn get_game_registry() -> &'static Mutex<GameRegistry> {
    GAME_REGISTRY.get_or_init(|| Mutex::new(GameRegistry::new()))
}

/// Stop the running UDP listener task, if any. Returns true if a task was stopped.
pub fn stop_listener() -> bool {
    let mut guard = get_listener_handle().lock().unwrap();
    if let Some(handle) = guard.take() {
        handle.abort();
        true
    } else {
        false
    }
}

const UDP_PORT: u16 = 20777;
const BUFFER_SIZE: usize = 2048;
const MAX_PACKET_RATE_HZ: f32 = 1000.0; // Cap at 1000 packets/sec

/// Statistics for monitoring the UDP connection
#[derive(Debug, Clone, Default)]
pub struct UdpStats {
    pub packets_received: u64,
    pub packets_parsed: u64,
    pub packets_failed: u64,
    pub bytes_received: u64,
    pub last_packet_time: Option<Instant>,
    pub packet_rate_hz: f32,
    pub detected_game: Option<GameId>,
}

/// Channel for sending parsed telemetry events to the broadcaster
pub type TelemetryTx = mpsc::UnboundedSender<TelemetryEvent>;
pub type TelemetryRx = mpsc::UnboundedReceiver<TelemetryEvent>;

/// Start the UDP listener and return a handle to the task and a receiver for telemetry events
pub async fn start_udp_listener(app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", UDP_PORT)).await?;
    info!("UDP listener started on port {}", UDP_PORT);

    // Channel for sending parsed events to the broadcaster
    let (tx, mut rx): (TelemetryTx, TelemetryRx) = mpsc::unbounded_channel();

    // Spawn the broadcaster task (handles Tauri event emission)
    let broadcaster_handle = tokio::spawn(broadcaster_task(app_handle, rx));

    // Spawn the UDP receive task
    let receive_handle = tokio::spawn(receive_task(socket, tx));

    // Store the handle globally so stop_telemetry can abort it.
    // Guard against double-start: only start if no listener is already running.
    let mut stored = get_listener_handle().lock().unwrap();
    if stored.is_some() {
        return Err("Telemetry listener already running".into());
    }

    // Return a handle that waits for both tasks
    let handle = tokio::spawn(async move {
        let _ = tokio::join!(receive_handle, broadcaster_handle);
    });

    *stored = Some(handle);

    Ok(())
}

/// Get the local IP addresses that the F1 game could connect to
/// Returns a list of non-loopback IPv4 addresses
pub fn get_local_ips() -> Vec<String> {
    let mut ips = Vec::new();

    // Get local IP by connecting to a remote address (most reliable method)
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                if let std::net::IpAddr::V4(ipv4) = local_addr.ip() {
                    if !ipv4.is_loopback() {
                        ips.push(ipv4.to_string());
                    }
                }
            }
        }
    }

    // Also try to get all local interfaces using local_ip_address crate
    if let Ok(local_ip) = local_ip_address::local_ip() {
        if let std::net::IpAddr::V4(ipv4) = local_ip {
            if !ipv4.is_loopback() && !ips.contains(&ipv4.to_string()) {
                ips.push(ipv4.to_string());
            }
        }
    }

    ips
}

/// Task that receives UDP packets and parses them
async fn receive_task(socket: UdpSocket, tx: TelemetryTx) {
    let mut buf = [0u8; BUFFER_SIZE];
    let mut stats = UdpStats::default();
    let mut last_rate_calc = Instant::now();
    let mut packets_since_rate_calc = 0u64;
    let mut detected_source_ip: Option<SocketAddr> = None;
    let mut last_detected_game: Option<GameId> = None;

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                stats.packets_received += 1;
                stats.bytes_received += len as u64;
                stats.last_packet_time = Some(Instant::now());
                packets_since_rate_calc += 1;

                // Detect and emit source IP on first packet from a new source
                if detected_source_ip.is_none() || detected_source_ip.unwrap() != addr {
                    detected_source_ip = Some(addr);
                    let ip_str = addr.ip().to_string();
                    let port = addr.port();
                    let _ = tx.send(TelemetryEvent::SourceIpDetected {
                        ip: ip_str.clone(),
                        port,
                    });
                    // Store in global state for API access
                    set_detected_source_ip(ip_str.clone(), port);
                    info!("Detected telemetry source: {}:{}", ip_str, port);
                }

                // Detect game from packet buffer
                {
                    let mut registry = get_game_registry().lock().unwrap();
                    if let Some(detection) = registry.detect_game(&buf[..len], addr) {
                        stats.detected_game = Some(detection.game_id);
                        if last_detected_game != Some(detection.game_id) {
                            last_detected_game = Some(detection.game_id);
                            let adapter = registry.get_adapter(detection.game_id);
                            let game_name = adapter.map(|a| a.display_name().to_string()).unwrap_or_else(|| detection.game_id.as_str().to_string());
                            let game_name_for_log = game_name.clone();
                            let _ = tx.send(TelemetryEvent::GameDetected {
                                game_id: detection.game_id.as_str().to_string(),
                                game_name,
                                source_ip: addr.ip().to_string(),
                                source_port: addr.port(),
                            });
                            info!("Detected game: {} from {}:{}", game_name_for_log, addr.ip(), addr.port());
                        }
                    }
                }

                // Calculate packet rate every second
                if last_rate_calc.elapsed() >= Duration::from_secs(1) {
                    stats.packet_rate_hz = packets_since_rate_calc as f32 / last_rate_calc.elapsed().as_secs_f32();
                    packets_since_rate_calc = 0;
                    last_rate_calc = Instant::now();

                    // Emit connection status periodically
                    let _ = tx.send(TelemetryEvent::ConnectionStatus {
                        connected: true,
                        packet_rate: stats.packet_rate_hz,
                        last_packet_time: stats.last_packet_time.map(|t| t.elapsed().as_millis() as u64).unwrap_or(0),
                    });
                }

                // Parse the packet
                let packet_data = &buf[..len];
                match parse_packet(packet_data) {
                    Ok(event) => {
                        stats.packets_parsed += 1;
                        if let Err(e) = tx.send(event) {
                            error!("Failed to send telemetry event: {}", e);
                            break; // Channel closed, exit
                        }
                    }
                    Err(ParseError::InvalidPacketId(id)) => {
                        // Unknown packet ID, just skip silently
                        debug!("Unknown packet ID: {}", id);
                    }
                    Err(e) => {
                        stats.packets_failed += 1;
                        warn!("Failed to parse packet from {}: {}", addr, e);
                    }
                }
            }
            Err(e) => {
                error!("UDP receive error: {}", e);
                // Brief pause before retrying
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
}

/// Task that receives parsed events and emits them via Tauri
async fn broadcaster_task(app_handle: tauri::AppHandle, mut rx: TelemetryRx) {
    // Throttle high-frequency events to avoid overwhelming the frontend
    let mut last_telemetry_emit = Instant::now();
    let mut last_status_emit = Instant::now();
    const TELEMETRY_THROTTLE_MS: u64 = 16; // ~60 FPS
    const STATUS_THROTTLE_MS: u64 = 1000; // 1 Hz

    while let Some(event) = rx.recv().await {
        let now = Instant::now();
        let should_emit = match &event {
            TelemetryEvent::ConnectionStatus { .. } => {
                now.duration_since(last_status_emit) >= Duration::from_millis(STATUS_THROTTLE_MS)
            }
            _ => now.duration_since(last_telemetry_emit) >= Duration::from_millis(TELEMETRY_THROTTLE_MS),
        };

        if should_emit {
            match &event {
                TelemetryEvent::ConnectionStatus { .. } => {
                    last_status_emit = now;
                }
                _ => {
                    last_telemetry_emit = now;
                }
            }

            // Emit to all frontend windows
            if let Err(e) = app_handle.emit("telemetry-event", &event) {
                error!("Failed to emit telemetry event: {}", e);
            }
        }
    }

    info!("Broadcaster task ended");
}

/// Get current UDP statistics
pub fn get_udp_stats() -> UdpStats {
    // In a real implementation, this would be shared state
    // For now, return default
    UdpStats::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_udp_bind() {
        let socket = UdpSocket::bind("127.0.0.1:0").await;
        assert!(socket.is_ok());
    }
}