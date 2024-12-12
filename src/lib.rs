pub mod models;
pub mod ui_automation;
pub mod screenshot;
pub mod commands;
pub mod tracking;

use tauri::http::Response;
use tauri::Manager;
use models::state::{TrackingState, WindowState};
use serde::Serialize;
use url::Url;

#[derive(Debug, Serialize, Clone)]
pub struct DeepLinkPayload {
    pub guide_id: String,
    pub position: i32,
    pub auth_token: String,
    pub total_steps: i32,  // Add this field
}

pub fn run() {
    let tracking_state = TrackingState::default();
    let tracking_state_clone_for_thread = tracking_state.clone();

    tauri::Builder::default()
        .manage(tracking_state)
        .manage(WindowState::default())
        .register_uri_scheme_protocol("matapass", move |app_handle, request| -> Result<Response, Box<dyn std::error::Error>> {
            // Get the URL from the request
            let url = request.uri().to_string();
            
            match Url::parse(&url) {
                Ok(parsed) => {
                    let params: std::collections::HashMap<_, _> = parsed.query_pairs().collect();
                    
                    let payload = DeepLinkPayload {
                        guide_id: params.get("guide_id")
                            .ok_or("Missing guide id".to_string())?
                            .to_string(),
                        position: params.get("position")
                            .ok_or("Missing position".to_string())?
                            .parse()
                            .map_err(|_| "Invalid position".to_string())?,
                        auth_token: params.get("auth_token")
                            .ok_or("Missing auth token".to_string())?
                            .to_string(),
                        total_steps: params.get("total_steps")  // Add this parameter
                            .ok_or("Missing total steps".to_string())?
                            .parse()
                            .map_err(|_| "Invalid total steps".to_string())?,
                    };

                    let window = if let Some(main_window) = app_handle.get_window("main") {
                        main_window
                    } else {
                        tauri::WindowBuilder::new(
                            app_handle,
                            "main",
                            tauri::WindowUrl::App("index.html".into())
                        ).build()?
                    };

                    window.emit("deep-link-payload", payload)
                        .expect("failed to emit event");
                    
                    Ok(Response::new(vec![]))
                },
                Err(_) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid URL"
                )))
            }
        })
        .setup(move |app| {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}