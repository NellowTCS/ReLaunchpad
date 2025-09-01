use plist::Value;
use serde::Serialize;
use std::fs;
use std::process::Command;

#[derive(Serialize)]
pub struct AppInfo {
    pub name: String,
    pub bundle_id: String,
    pub path: String,
    pub icon_path: Option<String>,
}

/// Lists all installed macOS applications
#[tauri::command]
pub fn list_apps() -> Vec<AppInfo> {
    let output = Command::new("mdfind")
        .arg("kMDItemContentType == 'com.apple.application-bundle'")
        .output()
        .expect("failed to run mdfind");

    let paths = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for path in paths.lines() {
        let plist_path = format!("{}/Contents/Info.plist", path);
        if let Ok(file) = fs::File::open(&plist_path) {
            if let Ok(info) = Value::from_reader_xml(file) {
                let dict = info.as_dictionary();

                let name = dict
                    .and_then(|d| d.get("CFBundleDisplayName").or(d.get("CFBundleName")))
                    .and_then(|v| v.as_string())
                    .unwrap_or("Unnamed")
                    .to_string();

                let bundle_id = dict
                    .and_then(|d| d.get("CFBundleIdentifier"))
                    .and_then(|v| v.as_string())
                    .unwrap_or("")
                    .to_string();

                let icon_path = dict
                    .and_then(|d| d.get("CFBundleIconFile"))
                    .and_then(|v| v.as_string())
                    .map(|s| {
                        let mut icon_file = s.to_string();
                        if !icon_file.ends_with(".icns") {
                            icon_file.push_str(".icns");
                        }
                        format!("{}/Contents/Resources/{}", path, icon_file)
                    });

                if !bundle_id.is_empty() {
                    results.push(AppInfo {
                        name,
                        bundle_id,
                        path: path.to_string(),
                        icon_path,
                    });
                }
            }
        }
    }

    results
}

/// Launches an app by its bundle identifier
#[tauri::command]
pub fn open_app(bundle_id: String) {
    let _ = Command::new("open").arg("-b").arg(bundle_id).spawn();
}
