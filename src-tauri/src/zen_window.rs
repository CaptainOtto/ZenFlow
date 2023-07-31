// Enum for each unique window we want to support. Will force you to implement the creation in get_zen_window
pub(crate) enum ZenWinType {
    Preferences,
}

pub(crate) trait ZenWindow {
    fn get_zen_window(&self, zen_win_type: ZenWinType) -> tauri::Window;
}

impl ZenWindow for tauri::AppHandle {
    // Passing in a ZenWinType Enum will create the window.
    fn get_zen_window(&self, zen_win_type: ZenWinType) -> tauri::Window {
        match zen_win_type {
            ZenWinType::Preferences => tauri::WindowBuilder::new(
                self,
                "Preferences",
                tauri::WindowUrl::App("index.html".into()),
            )
            .build()
            .expect("Failed to create Preferences window!"),
        }
    }
}
