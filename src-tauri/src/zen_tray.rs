// All TrayMenuItems are unique so, here represented as a Enum
pub(crate) enum ZenMenuItem {
    NextMini,
    NextLong,
    Skip,
    Pause,
    Reset,
    Preferences,
    Quit,
    Separator,
}

// For each TrayMenuItem Enum there must exist a match arm to create the Item.
pub(crate) fn init_tray() -> tauri::SystemTrayMenu {
    // Create the order of how you want the items in your tray to be.
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

    let mut tray_menu = tauri::SystemTrayMenu::new();
    for item_enum in zen_tray_item_order {
        match item_enum {
            ZenMenuItem::NextMini => {
                let item = tauri::CustomMenuItem::new(
                    "next_mini".to_string(),
                    "Next Mini Break in about 10 minutes",
                )
                .disabled();
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::NextLong => {
                let item = tauri::CustomMenuItem::new(
                    "next_long".to_string(),
                    "Next Long Break after 2 Mini Breaks",
                )
                .disabled();
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Skip => {
                let item = tauri::CustomMenuItem::new("skip".to_string(), "Skip to the next");
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Pause => {
                let item = tauri::CustomMenuItem::new("pause".to_string(), "Pause Breaks");
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Reset => {
                let item = tauri::CustomMenuItem::new("reset".to_string(), "Reset Breaks");
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Preferences => {
                let item = tauri::CustomMenuItem::new("preferences".to_string(), "Preferences");
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Quit => {
                let item = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
                tray_menu = tray_menu.add_item(item);
            }
            ZenMenuItem::Separator => {
                tray_menu = tray_menu.add_native_item(tauri::SystemTrayMenuItem::Separator);
            }
        }
    }
    tray_menu
}
