use std::sync::{Arc, Mutex};

use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
};

mod audio;
mod clipboard;
mod config;
mod hotkey;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize configuration
            let config = config::ConfigurationStore::load()?;
            
            // Create system tray menu
            let tray_menu = create_tray_menu();
            app.manage(SystemTray::new().with_menu(tray_menu));
            
            Ok(())
        })
        .system_tray(SystemTray::new())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                // Show settings window when tray icon is left-clicked
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "settings" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => (),
            },
            _ => (),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_tray_menu() -> SystemTrayMenu {
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let separator = SystemTrayMenuItem::separator();
    
    SystemTrayMenu::new()
        .add_item(settings)
        .add_item(separator)
        .add_item(quit)
}
