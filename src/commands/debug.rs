use tauri::Window;
use crate::DeepLinkPayload;

#[allow(non_snake_case)]
#[tauri::command]
pub fn debug_deep_link(window: Window, guideId: String, position: i32, authToken: String, totalSteps: i32) {
    println!("Debug deep link called with params:");
    println!("  guideId: {}", guideId);
    println!("  position: {}", position);
    println!("  authToken: {}", authToken);
    println!("  totalSteps: {}", totalSteps);

    let payload = DeepLinkPayload {
        guide_id: guideId,
        position,
        auth_token: authToken,
        total_steps: totalSteps,
    };
    
    match window.emit("deep-link-payload", payload) {
        Ok(_) => println!("Successfully emitted deep-link-payload event"),
        Err(e) => println!("Failed to emit deep-link-payload event: {}", e),
    }
}