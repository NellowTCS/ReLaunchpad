use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tauri::{AppHandle, Manager, Window};

#[derive(Serialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub bundle_id: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsage {
    pub bundle_id: String,
    pub launch_count: u32,
    pub last_launched: Option<String>,
}

/// MOCK: Lists fake applications for testing UI
#[tauri::command]
pub fn list_apps() -> Result<Vec<AppInfo>, String> {
    println!("list_apps() called (MOCK MODE)");

    // Create mock apps for testing
    let mock_apps = vec![
        AppInfo {
            name: "Safari".to_string(),
            bundle_id: "com.apple.Safari".to_string(),
            path: "/Applications/Safari.app".to_string(),
            icon_path: None,
            category: Some("Internet".to_string()),
            version: Some("16.0".to_string()),
        },
        AppInfo {
            name: "Mail".to_string(),
            bundle_id: "com.apple.mail".to_string(),
            path: "/Applications/Mail.app".to_string(),
            icon_path: None,
            category: Some("Productivity".to_string()),
            version: Some("16.0".to_string()),
        },
        AppInfo {
            name: "Photos".to_string(),
            bundle_id: "com.apple.Photos".to_string(),
            path: "/Applications/Photos.app".to_string(),
            icon_path: None,
            category: Some("Photography".to_string()),
            version: Some("8.0".to_string()),
        },
        AppInfo {
            name: "Messages".to_string(),
            bundle_id: "com.apple.MobileSMS".to_string(),
            path: "/Applications/Messages.app".to_string(),
            icon_path: None,
            category: Some("Social Networking".to_string()),
            version: Some("14.0".to_string()),
        },
        AppInfo {
            name: "Calendar".to_string(),
            bundle_id: "com.apple.iCal".to_string(),
            path: "/Applications/Calendar.app".to_string(),
            icon_path: None,
            category: Some("Productivity".to_string()),
            version: Some("11.0".to_string()),
        },
        AppInfo {
            name: "Music".to_string(),
            bundle_id: "com.apple.Music".to_string(),
            path: "/Applications/Music.app".to_string(),
            icon_path: None,
            category: Some("Music".to_string()),
            version: Some("1.3".to_string()),
        },
        AppInfo {
            name: "Notes".to_string(),
            bundle_id: "com.apple.Notes".to_string(),
            path: "/Applications/Notes.app".to_string(),
            icon_path: None,
            category: Some("Productivity".to_string()),
            version: Some("4.9".to_string()),
        },
        AppInfo {
            name: "Reminders".to_string(),
            bundle_id: "com.apple.reminders".to_string(),
            path: "/Applications/Reminders.app".to_string(),
            icon_path: None,
            category: Some("Productivity".to_string()),
            version: Some("7.0".to_string()),
        },
        AppInfo {
            name: "Maps".to_string(),
            bundle_id: "com.apple.Maps".to_string(),
            path: "/Applications/Maps.app".to_string(),
            icon_path: None,
            category: Some("Travel".to_string()),
            version: Some("5.0".to_string()),
        },
        AppInfo {
            name: "FaceTime".to_string(),
            bundle_id: "com.apple.FaceTime".to_string(),
            path: "/Applications/FaceTime.app".to_string(),
            icon_path: None,
            category: Some("Social Networking".to_string()),
            version: Some("5.0".to_string()),
        },
        AppInfo {
            name: "App Store".to_string(),
            bundle_id: "com.apple.AppStore".to_string(),
            path: "/Applications/App Store.app".to_string(),
            icon_path: None,
            category: Some("Utilities".to_string()),
            version: Some("3.0".to_string()),
        },
        AppInfo {
            name: "System Preferences".to_string(),
            bundle_id: "com.apple.systempreferences".to_string(),
            path: "/Applications/System Preferences.app".to_string(),
            icon_path: None,
            category: Some("Utilities".to_string()),
            version: Some("15.0".to_string()),
        },
        AppInfo {
            name: "Xcode".to_string(),
            bundle_id: "com.apple.dt.Xcode".to_string(),
            path: "/Applications/Xcode.app".to_string(),
            icon_path: None,
            category: Some("Developer Tools".to_string()),
            version: Some("14.0".to_string()),
        },
        AppInfo {
            name: "Terminal".to_string(),
            bundle_id: "com.apple.Terminal".to_string(),
            path: "/Applications/Utilities/Terminal.app".to_string(),
            icon_path: None,
            category: Some("Utilities".to_string()),
            version: Some("2.13".to_string()),
        },
        AppInfo {
            name: "Visual Studio Code".to_string(),
            bundle_id: "com.microsoft.VSCode".to_string(),
            path: "/Applications/Visual Studio Code.app".to_string(),
            icon_path: None,
            category: Some("Developer Tools".to_string()),
            version: Some("1.85".to_string()),
        },
    ];

    Ok(mock_apps)
}

#[tauri::command]
pub fn get_frequent_apps() -> Result<Vec<AppInfo>, String> {
    let usage_data = load_app_usage()?;
    let all_apps = list_apps()?;

    let app_map: HashMap<String, AppInfo> = all_apps
        .into_iter()
        .map(|app| (app.bundle_id.clone(), app))
        .collect();

    let mut frequent_apps: Vec<(AppInfo, u32)> = usage_data
        .into_iter()
        .filter_map(|usage| {
            app_map
                .get(&usage.bundle_id)
                .map(|app| (app.clone(), usage.launch_count))
        })
        .collect();

    frequent_apps.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(frequent_apps
        .into_iter()
        .take(20)
        .map(|(app, _)| app)
        .collect())
}

#[tauri::command]
pub fn track_app_usage(bundle_id: String) -> Result<(), String> {
    let mut usage_data: Vec<AppUsage> = load_app_usage().unwrap_or_default();

    if let Some(usage) = usage_data.iter_mut().find(|u| u.bundle_id == bundle_id) {
        usage.launch_count += 1;
        usage.last_launched = Some(chrono::Utc::now().to_rfc3339());
    } else {
        usage_data.push(AppUsage {
            bundle_id,
            launch_count: 1,
            last_launched: Some(chrono::Utc::now().to_rfc3339()),
        });
    }

    save_app_usage(&usage_data)?;
    Ok(())
}

fn load_app_usage() -> Result<Vec<AppUsage>, String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find home directory")?;
    let usage_file = format!("{}/.relaunchpad_usage.json", home_dir);

    if !Path::new(&usage_file).exists() {
        return Ok(Vec::new());
    }

    let contents =
        fs::read_to_string(&usage_file).map_err(|e| format!("Failed to read usage file: {}", e))?;

    serde_json::from_str(&contents).map_err(|e| format!("Failed to parse usage data: {}", e))
}

fn save_app_usage(usage_data: &[AppUsage]) -> Result<(), String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find home directory")?;
    let usage_file = format!("{}/.relaunchpad_usage.json", home_dir);

    let json = serde_json::to_string_pretty(usage_data)
        .map_err(|e| format!("Failed to serialize usage data: {}", e))?;

    fs::write(&usage_file, json).map_err(|e| format!("Failed to write usage file: {}", e))
}

/// MOCK: Just prints instead of actually opening
#[tauri::command]
pub fn open_app(bundle_id: String) -> Result<(), String> {
    println!("MOCK: Would open app with bundle ID: {}", bundle_id);
    Ok(())
}

#[tauri::command]
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub fn hide_main_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

/// MOCK: Returns empty data
#[tauri::command]
pub async fn read_icon_file(_path: String) -> Result<Vec<u8>, String> {
    Ok(Vec::new())
}

/// MOCK: Returns empty string
#[tauri::command]
pub async fn get_app_icon_base64(_icon_path: String) -> Result<String, String> {
    Ok(String::new())
}