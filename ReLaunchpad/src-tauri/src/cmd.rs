/// Shows the main window (for global shortcut) - Fixed for Tauri v2
#[tauri::command]
pub fn show_main_window(app: AppHandle) -> Result<(), String> {
    // Use get_webview_window instead of get_window for Tauri v2
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

/// Hides the main window
#[tauri::command]
pub fn hide_main_window(window: Window) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}use plist::Value;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use tauri::{AppHandle, Manager, Window};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub bundle_id: String,
    pub path: String,
    pub icon_path: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
}

// Define the AppUsage struct with proper serde derives
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppUsage {
    pub bundle_id: String,
    pub launch_count: u32,
    pub last_launched: Option<String>,
}

/// Lists all installed macOS applications
#[tauri::command]
pub fn list_apps() -> Result<Vec<AppInfo>, String> {
    let output = Command::new("mdfind")
        .arg("kMDItemContentType == 'com.apple.application-bundle'")
        .output()
        .map_err(|e| format!("Failed to run mdfind: {}", e))?;

    if !output.status.success() {
        return Err("mdfind command failed".to_string());
    }

    let paths = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for path in paths.lines() {
        if path.trim().is_empty() {
            continue;
        }

        if let Some(app_info) = parse_app_info(path) {
            results.push(app_info);
        }
    }

    // Sort by name for consistency
    results.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    
    Ok(results)
}

fn parse_app_info(app_path: &str) -> Option<AppInfo> {
    let plist_path = format!("{}/Contents/Info.plist", app_path);
    
    // Check if the plist file exists
    if !Path::new(&plist_path).exists() {
        return None;
    }

    let file = fs::File::open(&plist_path).ok()?;
    let info = Value::from_reader_xml(file).ok()?;
    let dict = info.as_dictionary()?;

    // Get app name (prefer display name over bundle name)
    let name = dict
        .get("CFBundleDisplayName")
        .or_else(|| dict.get("CFBundleName"))
        .and_then(|v| v.as_string())
        .filter(|s| !s.is_empty())
        .unwrap_or("Unnamed")
        .to_string();

    // Get bundle identifier
    let bundle_id = dict
        .get("CFBundleIdentifier")
        .and_then(|v| v.as_string())
        .filter(|s| !s.is_empty())?
        .to_string();

    // Get version
    let version = dict
        .get("CFBundleShortVersionString")
        .or_else(|| dict.get("CFBundleVersion"))
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());

    // Get category
    let category = dict
        .get("LSApplicationCategoryType")
        .and_then(|v| v.as_string())
        .map(|s| format_app_category(s))
        .or_else(|| guess_category_from_path(app_path));

    // Get icon path
    let icon_path = get_app_icon_path(dict, app_path);

    Some(AppInfo {
        name,
        bundle_id,
        path: app_path.to_string(),
        icon_path,
        category,
        version,
    })
}

fn format_app_category(category: &str) -> String {
    match category {
        "public.app-category.productivity" => "Productivity".to_string(),
        "public.app-category.graphics-design" => "Graphics & Design".to_string(),
        "public.app-category.developer-tools" => "Developer Tools".to_string(),
        "public.app-category.entertainment" => "Entertainment".to_string(),
        "public.app-category.education" => "Education".to_string(),
        "public.app-category.lifestyle" => "Lifestyle".to_string(),
        "public.app-category.utilities" => "Utilities".to_string(),
        "public.app-category.games" => "Games".to_string(),
        "public.app-category.social-networking" => "Social Networking".to_string(),
        "public.app-category.finance" => "Finance".to_string(),
        "public.app-category.photography" => "Photography".to_string(),
        "public.app-category.music" => "Music".to_string(),
        "public.app-category.video" => "Video".to_string(),
        _ => "Other".to_string(),
    }
}

fn guess_category_from_path(path: &str) -> Option<String> {
    if path.contains("Utilities") {
        Some("Utilities".to_string())
    } else if path.contains("Games") {
        Some("Games".to_string())
    } else if path.contains("Graphics") || path.contains("Design") {
        Some("Graphics & Design".to_string())
    } else {
        None
    }
}

fn get_app_icon_path(dict: &plist::Dictionary, app_path: &str) -> Option<String> {
    // Try different icon keys
    let icon_keys = [
        "CFBundleIconFile",
        "CFBundleIconName",
        "CFBundleIcons",
    ];

    for key in &icon_keys {
        if let Some(icon_value) = dict.get(*key) {
            if let Some(icon_name) = icon_value.as_string() {
                let mut icon_file = icon_name.to_string();
                
                // Ensure .icns extension
                if !icon_file.ends_with(".icns") {
                    icon_file.push_str(".icns");
                }
                
                let icon_path = format!("{}/Contents/Resources/{}", app_path, icon_file);
                
                // Check if the icon file actually exists
                if Path::new(&icon_path).exists() {
                    return Some(icon_path);
                }
            }
        }
    }

    // Try to find any .icns file in Resources folder as fallback
    let resources_path = format!("{}/Contents/Resources", app_path);
    if let Ok(entries) = fs::read_dir(&resources_path) {
        for entry in entries.flatten() {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".icns") {
                    let icon_path = format!("{}/{}", resources_path, filename);
                    return Some(icon_path);
                }
            }
        }
    }

    None
}

/// Gets frequently used apps based on usage tracking
#[tauri::command]
pub fn get_frequent_apps() -> Result<Vec<AppInfo>, String> {
    let usage_data = load_app_usage()?;
    let all_apps = list_apps()?;
    
    // Create a map for quick lookup
    let app_map: HashMap<String, AppInfo> = all_apps
        .into_iter()
        .map(|app| (app.bundle_id.clone(), app))
        .collect();
    
    let mut frequent_apps: Vec<(AppInfo, u32)> = usage_data
        .into_iter()
        .filter_map(|usage| {  // Removed type annotation here
            app_map.get(&usage.bundle_id)
                .map(|app| (app.clone(), usage.launch_count))
        })
        .collect();
    
    // Sort by launch count (descending) and take top 20
    frequent_apps.sort_by(|a, b| b.1.cmp(&a.1));
    
    Ok(frequent_apps.into_iter()
        .take(20)
        .map(|(app, _)| app)
        .collect())
}

/// Tracks app usage for frequent apps feature
#[tauri::command]
pub fn track_app_usage(bundle_id: String) -> Result<(), String> {
    // Fixed syntax error: removed invalid generic syntax
    let mut usage_data: Vec<AppUsage> = load_app_usage().unwrap_or_default();
    
    // Find existing entry or create new one
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

// Fixed function signature - removed invalid generic syntax
fn load_app_usage() -> Result<Vec<AppUsage>, String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find home directory")?;
    let usage_file = format!("{}/.relaunchpad_usage.json", home_dir);
    
    if !Path::new(&usage_file).exists() {
        return Ok(Vec::new());
    }
    
    let contents = fs::read_to_string(&usage_file)
        .map_err(|e| format!("Failed to read usage file: {}", e))?;
    
    serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse usage data: {}", e))
}

fn save_app_usage(usage_data: &[AppUsage]) -> Result<(), String> {
    let home_dir = std::env::var("HOME").map_err(|_| "Could not find home directory")?;
    let usage_file = format!("{}/.relaunchpad_usage.json", home_dir);
    
    let json = serde_json::to_string_pretty(usage_data)
        .map_err(|e| format!("Failed to serialize usage data: {}", e))?;
    
    fs::write(&usage_file, json)
        .map_err(|e| format!("Failed to write usage file: {}", e))
}

#[tauri::command]
pub fn open_app(bundle_id: String) -> Result<(), String> {
    if bundle_id.is_empty() {
        return Err("Bundle ID cannot be empty".to_string());
    }

    let output = Command::new("open")
        .arg("-b")
        .arg(&bundle_id)
        .output()
        .map_err(|e| format!("Failed to execute open command: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to open app '{}': {}", bundle_id, error_msg));
    }

    Ok(())
}