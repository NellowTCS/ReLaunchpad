// Prevents additional console window on macOS in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;

use cmd::{get_frequent_apps, hide_main_window, list_apps, open_app, show_main_window, track_app_usage, get_app_icon_base64, read_icon_file};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_apps,
            get_frequent_apps,
            open_app,
            track_app_usage,
            show_main_window,
            hide_main_window,
            get_app_icon_base64,
            read_icon_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running ReLaunchpad");
}

fn main() {
    run();
}