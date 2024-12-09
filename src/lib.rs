pub mod models;
pub mod ui_automation;
pub mod screenshot;
pub mod commands;
pub mod tracking;

use models::state::{TrackingState, WindowState};

pub fn run() {
    let tracking_state = TrackingState::default();
    let tracking_state_clone_for_thread = tracking_state.clone();

    tauri::Builder::default()
        .manage(tracking_state)
        .manage(WindowState::default())
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
            commands::tracking::enter_compact_mode,  // Add this line
            commands::guide::load_guides,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}