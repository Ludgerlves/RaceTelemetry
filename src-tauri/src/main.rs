#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_telemetry,
            commands::stop_telemetry,
            commands::get_session_info,
            commands::export_session,
            commands::get_available_drivers,
            commands::set_active_driver,
            commands::get_settings,
            commands::save_settings,
            commands::clear_session_data,
            commands::list_sessions,
            commands::get_session,
            commands::delete_session,
            commands::save_current_session,
            commands::get_detected_source_ip,
            commands::get_local_ips,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = telemetry::udp_listener::start_udp_listener(app_handle).await {
                    tracing::error!("UDP listener error: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}

// Declare modules
mod commands;
mod telemetry;