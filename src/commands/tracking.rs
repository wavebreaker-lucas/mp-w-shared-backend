use tauri::{Window, State, Size, PhysicalSize, Position, PhysicalPosition};
use crate::models::state::{TrackingState, WindowState, STATE_RUNNING, STATE_PAUSED, STATE_STOPPED};

#[tauri::command]
pub async fn enter_compact_mode(
    window: Window,
    window_state: State<'_, WindowState>,
) -> Result<(), String> {
    let current_size = window.outer_size().map_err(|e| e.to_string())?;
    window_state.set_size(current_size);

    window.set_size(Size::Physical(PhysicalSize {
        width: crate::models::state::COMPACT_WIDTH,
        height: crate::models::state::COMPACT_HEIGHT,
    })).map_err(|e| e.to_string())?;
    
    if let Ok(monitor) = window.current_monitor() {
        if let Some(monitor) = monitor {
            let monitor_size = monitor.size();
            let window_size = window.outer_size().map_err(|e| e.to_string())?;
            
            let center_x = monitor.position().x + 
                ((monitor_size.width as i32 - window_size.width as i32) / 2);
            let center_y = monitor.position().y + 
                ((monitor_size.height as i32 - window_size.height as i32) / 2);
            
            window.set_position(Position::Physical(PhysicalPosition {
                x: center_x,
                y: center_y
            })).map_err(|e| e.to_string())?;
        }
    }

    window.set_always_on_top(true).map_err(|e| e.to_string())?;
    
    window.emit("recording-mode", true).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_pause(
    window: Window,
    tracking_state: State<'_, TrackingState>,
) -> Result<bool, String> {
    let current_state = tracking_state.get_state();
    
    match current_state {
        STATE_RUNNING => {
            tracking_state.set_state(STATE_PAUSED);
            window.emit("recording-paused", true).map_err(|e| e.to_string())?;
            window.unminimize().map_err(|e| e.to_string())?;
            window.show().map_err(|e| e.to_string())?;
            Ok(true)
        },
        STATE_PAUSED => {
            tracking_state.set_state(STATE_RUNNING);
            window.emit("recording-paused", false).map_err(|e| e.to_string())?;
            window.minimize().map_err(|e| e.to_string())?;
            Ok(false)
        },
        _ => Ok(false),
    }
}

#[tauri::command]
pub async fn start_tracking(
    window: Window,
    tracking_state: State<'_, TrackingState>,
) -> Result<(), String> {
    tracking_state.set_state(STATE_RUNNING);
    window.emit("recording-mode", true).map_err(|e| e.to_string())?;
    window.minimize().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn stop_tracking(
    window: Window,
    tracking_state: State<'_, TrackingState>,
    window_state: State<'_, WindowState>,
) -> Result<(), String> {
    tracking_state.set_state(STATE_STOPPED);

    window.unminimize().map_err(|e| e.to_string())?;
    window.show().map_err(|e| e.to_string())?;

    if let Some(original_size) = window_state.get_size() {
        window.set_size(Size::Physical(original_size)).map_err(|e| e.to_string())?;
    }
    
    if let Ok(monitor) = window.current_monitor() {
        if let Some(monitor) = monitor {
            let monitor_size = monitor.size();
            let window_size = window.outer_size().map_err(|e| e.to_string())?;
            
            let center_x = monitor.position().x + 
                ((monitor_size.width as i32 - window_size.width as i32) / 2);
            let center_y = monitor.position().y + 
                ((monitor_size.height as i32 - window_size.height as i32) / 2);
            
            window.set_position(Position::Physical(PhysicalPosition {
                x: center_x,
                y: center_y
            })).map_err(|e| e.to_string())?;
        }
    }
    
    window.set_always_on_top(false).map_err(|e| e.to_string())?;
    window.set_decorations(true).map_err(|e| e.to_string())?;
    
    window.emit("recording-mode", false).map_err(|e| e.to_string())?;
    Ok(())
}