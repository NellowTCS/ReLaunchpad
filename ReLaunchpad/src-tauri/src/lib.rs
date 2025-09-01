// Prevents additional console window on macOS in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
use cmd::{
    get_frequent_apps, hide_main_window, list_apps, open_app, show_main_window, track_app_usage,
};
use tauri::Manager;
use tauri::WindowEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // Configure window behavior
            window.set_always_on_top(true).unwrap();

            // Remove window decorations for cleaner look
            window.set_decorations(false).unwrap();

            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }
            
            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide window instead of closing
                    api.prevent_close();
                    window.hide().unwrap();
                }
                WindowEvent::Focused(false) => {
                    // Hide window when it loses focus (like Spotlight)
                    window.hide().unwrap();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            list_apps,
            get_frequent_apps,
            open_app,
            track_app_usage,
            show_main_window,
            hide_main_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running ReLaunchpad");
}
