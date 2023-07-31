// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{SystemTray, SystemTrayEvent};

mod zen_window;
use zen_window::{ZenWinType, ZenWindow};

mod zen_tray;
use zen_tray::{init_tray_from_vec, ZenMenuItem};

fn main() {
    // Create the order of how you want the items in your tray to be.
    // Maybe move this to zen_tray.rs?
    let zen_tray_item_order: Vec<ZenMenuItem> = vec![
        ZenMenuItem::NextMini,
        ZenMenuItem::NextLong,
        ZenMenuItem::Separator,
        ZenMenuItem::Skip,
        ZenMenuItem::Pause,
        ZenMenuItem::Reset,
        ZenMenuItem::Separator,
        ZenMenuItem::Preferences,
        ZenMenuItem::Separator,
        ZenMenuItem::Quit,
    ];

    // Init the tray from the order list above.
    let tray_menu = init_tray_from_vec(zen_tray_item_order);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
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
