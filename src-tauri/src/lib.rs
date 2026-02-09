mod hardware;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let backend = hardware::detect_gpu_backend();
    println!("🚀 Backend Detected: {:?}", backend);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .setup(move |app| {
            // Store backend info in app state for the STT Factory later
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
