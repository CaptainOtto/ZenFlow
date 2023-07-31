// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{SystemTray, SystemTrayEvent};

mod zen_window;
use zen_window::{ZenWinType, ZenWindow};

mod zen_tray;

fn main() {
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(zen_tray::init_tray()))
        .on_system_tray_event(|app, event| match event {
            // Can we somehow move this to zen_tray.rs?
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "preferences" => {
                    let _window = app.get_zen_window(ZenWinType::Preferences);
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
