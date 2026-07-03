pub mod audio;
mod cleanup;
pub mod clipboard;
mod commands;
pub mod config;
pub mod hardware;
mod history;
pub mod hotkey;
mod metrics;
pub mod providers;
mod ptt;
mod state;

use tauri::Manager;

pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let backend = hardware::detect_gpu_backend();
    log::info!("GPU Backend Detected: {:?}", backend);

    let state = state::AppState::new(backend);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .manage(state)
        .setup(|app| {
            // Set up system tray
            use tauri::menu::{Menu, MenuItem};
            use tauri::tray::TrayIconBuilder;
            
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &hide, &quit])?;
            
            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Voice Assistant")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { button, .. } = event {
                        if button == tauri::tray::MouseButton::Left {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;
            
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::update_settings,
            commands::list_audio_devices,
            commands::start_recording,
            commands::stop_recording,
            commands::get_audio_levels,
            commands::transcribe,
            commands::paste_text,
            commands::register_hotkey,
            commands::unregister_hotkey,
            commands::get_gpu_backend,
            commands::get_history,
            commands::search_history,
            commands::delete_history_entry,
            commands::clear_history,
            commands::get_usage_metrics,
            commands::get_resource_usage,
            commands::cleanup_text,
            commands::record_metric,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}