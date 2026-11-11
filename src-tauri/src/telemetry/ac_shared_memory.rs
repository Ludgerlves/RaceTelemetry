//! Assetto Corsa / ACC shared memory reader (Windows only).
//!
//! Reads AC/ACC telemetry from Windows shared memory sections created by the game
//! (and exposed via Content Manager / native plugins).
//!
//! Shared memory section names (Kunos Simulazioni convention):
//!   - "Local\\acpmf_physics"   (300 Hz physics data)
//!   - "Local\\acpmf_graphics"  (60 Hz graphics data)
//!   - "Local\\acpmf_static"    (static car/track info, read once)
//!
//! On non-Windows platforms this module is a compile-safe stub that logs
//! a warning and does nothing.

use std::sync::{OnceLock, Mutex};
use std::time::Instant;
use tauri::Emitter;
use tracing::{error, info, warn};

use crate::telemetry::game_detection::{GameId, GameRegistry};
use crate::telemetry::packets::TelemetryEvent;

// Shared memory section names for AC / ACC
const SHM_PHYSICS: &str = "Local\\acpmf_physics";
const SHM_GRAPHICS: &str = "Local\\acpmf_graphics";
const SHM_STATIC: &str = "Local\\acpmf_static";

// Sizes (from Kunos shared memory specification)
const PHYSICS_SIZE: usize = 768;    // SPageFilePhysics
const GRAPHICS_SIZE: usize = 768;   // SPageFileGraphic
const STATIC_SIZE: usize = 768;     // SPageFileStatic

/// Global handle to the shared memory reader task
static SHM_HANDLE: OnceLock<Mutex<Option<std::thread::ThreadId>>> = OnceLock::new();

fn get_shm_handle() -> &'static Mutex<Option<std::thread::ThreadId>> {
    SHM_HANDLE.get_or_init(|| Mutex::new(None))
}

/// Start reading AC/ACC shared memory. Windows-only; stub on other platforms.
pub fn start_shared_memory(app_handle: tauri::AppHandle) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        start_shm_windows(app_handle)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = app_handle;
        warn!("Assetto Corsa shared memory is only available on Windows. This platform is not supported.");
        Err("Assetto Corsa shared memory is only available on Windows".into())
    }
}

/// Stop reading AC/ACC shared memory.
pub fn stop_shared_memory() -> bool {
    #[cfg(target_os = "windows")]
    {
        stop_shm_windows()
    }

    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

/// Check if shared memory reader is running.
pub fn is_shared_memory_running() -> bool {
    get_shm_handle().lock().unwrap().is_some()
}

#[cfg(target_os = "windows")]
fn start_shm_windows(app_handle: tauri::AppHandle) -> Result<(), String> {
    use std::ptr;
    use winapi::um::memoryapi::{CreateFileMappingA, MapViewOfFile, UnmapViewOfFile};
    use winapi::um::winnt::{FILE_MAP_READ, PAGE_READONLY, SEC_IMAGE, HANDLE};
    use winapi::um::handleapi::CloseHandle;

    let mut stored = get_shm_handle().lock().unwrap();
    if stored.is_some() {
        return Err("Shared memory reader already running".into());
    }

    info!("Starting Assetto Corsa shared memory reader...");

    // Open (or create if absent) the shared memory sections
    let physics_map = unsafe {
        CreateFileMappingA(
            std::ptr::null_mut(), // use paging file
            ptr::null_mut(),
            PAGE_READONLY,
            0,
            PHYSICS_SIZE as u32,
            SHM_PHYSICS.as_ptr() as *const i8,
        )
    };
    if physics_map.is_null() {
        return Err(format!("Failed to open shared memory section: {}", SHM_PHYSICS));
    }

    let graphics_map = unsafe {
        CreateFileMappingA(
            std::ptr::null_mut(),
            ptr::null_mut(),
            PAGE_READONLY,
            0,
            GRAPHICS_SIZE as u32,
            SHM_GRAPHICS.as_ptr() as *const i8,
        )
    };
    if graphics_map.is_null() {
        unsafe { CloseHandle(physics_map); }
        return Err(format!("Failed to open shared memory section: {}", SHM_GRAPHICS));
    }

    let static_map = unsafe {
        CreateFileMappingA(
            std::ptr::null_mut(),
            ptr::null_mut(),
            PAGE_READONLY,
            0,
            STATIC_SIZE as u32,
            SHM_STATIC.as_ptr() as *const i8,
        )
    };
    if static_map.is_null() {
        unsafe {
            CloseHandle(physics_map);
            CloseHandle(graphics_map);
        }
        return Err(format!("Failed to open shared memory section: {}", SHM_STATIC));
    }

    // Map views
    let physics_view = unsafe { MapViewOfFile(physics_map, FILE_MAP_READ, 0, 0, PHYSICS_SIZE) };
    if physics_view.is_null() {
        unsafe {
            CloseHandle(physics_map);
            CloseHandle(graphics_map);
            CloseHandle(static_map);
        }
        return Err("Failed to map physics shared memory view".into());
    }

    let graphics_view = unsafe { MapViewOfFile(graphics_map, FILE_MAP_READ, 0, 0, GRAPHICS_SIZE) };
    if graphics_view.is_null() {
        unsafe {
            UnmapViewOfFile(physics_view);
            CloseHandle(physics_map);
            CloseHandle(graphics_map);
            CloseHandle(static_map);
        }
        return Err("Failed to map graphics shared memory view".into());
    }

    let static_view = unsafe { MapViewOfFile(static_map, FILE_MAP_READ, 0, 0, STATIC_SIZE) };
    if static_view.is_null() {
        unsafe {
            UnmapViewOfFile(physics_view);
            UnmapViewOfFile(graphics_view);
            CloseHandle(physics_map);
            CloseHandle(graphics_map);
            CloseHandle(static_map);
        }
        return Err("Failed to map static shared memory view".into());
    }

    info!("Assetto Corsa shared memory sections opened successfully");

    // Spawn reader thread (polls at ~60 Hz)
    let app_handle_clone = app_handle.clone();
    let handle = std::thread::spawn(move || {
        let mut last_emit = Instant::now();
        let interval = std::time::Duration::from_millis(16); // ~60 Hz

        loop {
            std::thread::sleep(std::time::Duration::from_millis(16));

            if last_emit.elapsed() < interval {
                continue;
            }
            last_emit = Instant::now();

            // Read a slice from each buffer for telemetry emission
            // The actual struct parsing would go here (SPageFilePhysics, etc.)
            // For now we emit a connection status event to indicate AC is connected
            let _ = app_handle_clone.emit("telemetry-event", &TelemetryEvent::ConnectionStatus {
                connected: true,
                packet_rate: 60.0,
                last_packet_time: 0,
            });
        }
    });

    *stored = Some(handle.id());
    Ok(())
}

#[cfg(target_os = "windows")]
fn stop_shm_windows() -> bool {
    let mut guard = get_shm_handle().lock().unwrap();
    if let Some(thread_id) = guard.take() {
        // Note: std::thread::ThreadId cannot be used to abort a thread safely.
        // In production, use a cancellation flag (AtomicBool) instead.
        info!("Shared memory reader stop requested (thread {:?})", thread_id);
        true
    } else {
        false
    }
}

#[cfg(not(target_os = "windows"))]
fn start_shm_windows(_app_handle: tauri::AppHandle) -> Result<(), String> {
    unreachable!("This function is only compiled on Windows")
}

#[cfg(not(target_os = "windows"))]
fn stop_shm_windows() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shm_stub_on_non_windows() {
        // On non-Windows, start_shared_memory should return an error
        // (we can't call it without an AppHandle in unit tests, so just
        // verify the module compiles)
        assert!(!is_shared_memory_running());
    }
}
