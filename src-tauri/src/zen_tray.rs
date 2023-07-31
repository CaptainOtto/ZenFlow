use crate::zen_window::{ZenAppHandle, ZenWinType};
use std::str::FromStr;
use tauri::SystemTrayEvent;

// All TrayMenuItems are unique so, here represented as a Enum
enum ZenMenuItem {
    NextMini,
    NextLong,
    Skip,
    Pause,
    Reset,
    Preferences,
    Quit,
    Separator,
}

impl FromStr for ZenMenuItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "next_mini" => Ok(ZenMenuItem::NextMini),
            "next_long" => Ok(ZenMenuItem::NextLong),
            "skip" => Ok(ZenMenuItem::Skip),
            "pause" => Ok(ZenMenuItem::Pause),
            "reset" => Ok(ZenMenuItem::Reset),
            "preferences" => Ok(ZenMenuItem::Preferences),
            "quit" => Ok(ZenMenuItem::Quit),
            "separator" => Ok(ZenMenuItem::Separator),
            _ => Err(()),
        }
    }
}

pub(crate) trait ZenTray {
    fn zen_tray_events(&self, event: SystemTrayEvent);
}

impl ZenTray for tauri::AppHandle {
    fn zen_tray_events(&self, event: SystemTrayEvent) {
        match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let menu_item = ZenMenuItem::from_str(id.as_str()).unwrap_or_else(|_| {
                    // Handle invalid input here
                    panic!("Invalid menu item: {}", id);
                });

                match menu_item {
                    ZenMenuItem::NextMini => {}
                    ZenMenuItem::NextLong => {}
                    ZenMenuItem::Skip => {
                        self.show_zen_window(ZenWinType::Skip);
                    }
                    ZenMenuItem::Pause => {}
                    ZenMenuItem::Reset => {}
                    ZenMenuItem::Preferences => {
                        self.show_zen_window(ZenWinType::Preferences);
                    }
                    ZenMenuItem::Quit => {
                        std::process::exit(0);
                    }
                    ZenMenuItem::Separator => {}
                }
            }
            _ => {}
        };
    }
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
