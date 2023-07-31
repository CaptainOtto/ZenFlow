// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod zen_tray;
mod zen_window;

use crate::zen_tray::ZenTray;
use tauri::SystemTray;

fn main() {
    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(zen_tray::init_tray()))
        .on_system_tray_event(tauri::AppHandle::zen_tray_events)
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
