use tauri::{Error, Window, WindowEvent};

// Enum for each unique window we want to support. Will force you to implement the creation in get_zen_window
pub(crate) enum ZenWinType {
    Skip,
    Preferences,
}

trait ToStr {
    fn to_url(&self) -> &str;
    fn to_str(&self) -> &str;
    fn to_string(&self) -> String;
}

impl ToStr for ZenWinType {
    fn to_url(&self) -> &str {
        match self {
            ZenWinType::Preferences => "index.html",
            ZenWinType::Skip => "index.html",
        }
    }

    fn to_str(&self) -> &str {
        match self {
            ZenWinType::Preferences => "Preferences",
            ZenWinType::Skip => "Skip",
        }
    }
    fn to_string(&self) -> String {
        match self {
            ZenWinType::Preferences => "Preferences".to_string(),
            ZenWinType::Skip => "Skip".to_string(),
        }
    }
}

trait ZenWindow {
    fn toggle_show_hide(&self) -> Result<(), Error>;
}

impl ZenWindow for Window {
    fn toggle_show_hide(&self) -> Result<(), Error> {
        if self.is_visible().expect("Window visibility check failed.") {
            self.hide()
        } else {
            self.show()
        }
    }
}

pub(crate) trait ZenAppHandle {
    fn show_zen_window(&self, zen_win_type: ZenWinType) -> tauri::Window;
}

impl ZenAppHandle for tauri::AppHandle {
    // Passing in a ZenWinType Enum will create the window.
    fn show_zen_window(&self, zen_win_type: ZenWinType) -> tauri::Window {
        // Function for creating the new base window
        let create_new_window = || {
            let new_window = tauri::WindowBuilder::new(
                self,
                zen_win_type.to_str(),
                tauri::WindowUrl::App(zen_win_type.to_url().into()),
            )
            .title(zen_win_type.to_str())
            .center()
            .resizable(false)
            .visible(false)
            .build()
            .expect(&format!(
                "Failed to create {} window",
                zen_win_type.to_str()
            ));

            // Necessary clone for using hide in event
            let window_e_closure = new_window.clone();
            // Listen for the CloseRequested event on the window.
            new_window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    _ = window_e_closure.hide();
                }
                _ => {}
            });

            return new_window;
        };

        let window = tauri::Manager::get_window(self, zen_win_type.to_str())
            .unwrap_or_else(create_new_window);

        _ = window.toggle_show_hide();
        _ = window.center();
        window
    }
}
