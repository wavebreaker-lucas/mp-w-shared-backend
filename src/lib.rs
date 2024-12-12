pub mod models;
pub mod ui_automation;
pub mod screenshot;
pub mod commands;
pub mod tracking;
mod protocol;

use std::fs;
use tauri::Manager;
use models::state::{TrackingState, WindowState};
use serde::{Serialize, Deserialize};
use url::Url;
use dirs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeepLinkPayload {
    pub guide_id: String,
    pub auth_token: String,
    pub total_steps: i32,
}

impl DeepLinkPayload {
    fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app_data_dir = dirs::data_dir()
            .ok_or("Failed to get app data directory")?
            .join("MataPass");
        
        fs::create_dir_all(&app_data_dir)?;
        let file_path = app_data_dir.join("deep_link.json");
        let json = serde_json::to_string(&self)?;
        fs::write(file_path, json)?;
        Ok(())
    }

    fn load_from_disk() -> Option<Self> {
        let file_path = dirs::data_dir()?.join("MataPass").join("deep_link.json");
        let content = fs::read_to_string(&file_path).ok()?;
        let data: DeepLinkPayload = serde_json::from_str(&content).ok()?;
        // Clean up after reading
        let _ = fs::remove_file(file_path);
        Some(data)
    }
}

#[tauri::command]
async fn get_launch_details() -> Result<Option<DeepLinkPayload>, String> {
    Ok(DeepLinkPayload::load_from_disk())
}

pub fn run() {
    let tracking_state = TrackingState::default();
    let tracking_state_clone_for_thread = tracking_state.clone();

    tauri::Builder::default()
        .manage(tracking_state)
        .manage(WindowState::default())
        .setup(move |app| {
            // Register protocol handler
            if let Err(e) = protocol::register_protocol_handler() {
                eprintln!("Failed to register protocol handler: {}", e);
            }

            // Prepare deep link plugin first
            tauri_plugin_deep_link::prepare("matapass");

            // Then register deep link handler
            let handle = app.handle().clone();
            tauri_plugin_deep_link::register("matapass", move |request| {
                if let Ok(parsed) = Url::parse(&request) {
                    let params: std::collections::HashMap<_, _> = parsed.query_pairs().collect();
                    
                    let payload = DeepLinkPayload {
                        guide_id: params.get("guide_id")
                            .map(|s| s.to_string())
                            .unwrap_or_default(),
                        auth_token: params.get("auth_token")
                            .map(|s| s.to_string())
                            .unwrap_or_default(),
                        total_steps: params.get("total_steps")
                            .and_then(|s| s.parse().ok())
                            .unwrap_or_default(),
                    };

                    // Save payload to disk before handling
                    if let Err(e) = payload.save_to_disk() {
                        eprintln!("Failed to save deep link data: {}", e);
                    }

                    if let Some(window) = handle.get_window("main") {
                        window.emit("deep-link-payload", payload).ok();
                    } else {
                        if let Ok(window) = tauri::WindowBuilder::new(
                            &handle,
                            "main",
                            tauri::WindowUrl::App("index.html".into())
                        ).build() {
                            window.emit("deep-link-payload", payload).ok();
                        }
                    }
                }
            })?;

            tracking::loop_handler::setup_tracking_loop(
                app.handle(),
                tracking_state_clone_for_thread
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::tracking::start_tracking,
            commands::tracking::stop_tracking,
            commands::tracking::toggle_pause,
            commands::tracking::enter_compact_mode,
            commands::guide::load_guides,
            commands::debug::debug_deep_link,
            get_launch_details, // Add the new command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}