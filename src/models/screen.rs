// models/screen.rs
use serde::{Serialize, Deserialize};
use crate::ui_automation::utils::get_screen_size;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ScreenContext {
    pub width: i32,
    pub height: i32,
}

impl ScreenContext {
    pub fn new() -> Self {
        let (width, height) = get_screen_size();
        Self { width, height }
    }
}