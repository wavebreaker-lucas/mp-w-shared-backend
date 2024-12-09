use crate::models::Guide;
use tauri::command;

#[command]
pub async fn load_guides() -> Result<Vec<Guide>, String> {
    // For now, return an empty vector
    // Later you can implement actual guide loading from file/database
    Ok(Vec::new())
}