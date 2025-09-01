mod cmd;

use cmd::{list_apps, open_app};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_apps, open_app])
        .run(tauri::generate_context!())
        .expect("error while running ReLaunchpad");
}
