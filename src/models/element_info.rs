use serde::{Serialize, Deserialize};
use std::fmt;

use super::screen::ScreenContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionCategory {
    Click,
    Keystroke,
    Manual,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub screen_context: ScreenContext,
    pub name: String,
    pub control_type: String,
    pub automation_id: String,
    pub class_name: String,
    pub window_title: String,
    pub parent_name: String,
    pub action_type: String,
    pub action_category: ActionCategory,
    pub timestamp: String,
    pub screenshot: Option<String>,
    pub value: String,
    pub state: String,
    pub help_text: String,
}

impl fmt::Debug for ElementInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementInfo")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("screen_context", &self.screen_context)
            .field("name", &self.name)
            .field("control_type", &self.control_type)
            .field("automation_id", &self.automation_id)
            .field("class_name", &self.class_name)
            .field("window_title", &self.window_title)
            .field("parent_name", &self.parent_name)
            .field("action_type", &self.action_type)
            .field("action_category", &self.action_category)
            .field("timestamp", &self.timestamp)
            .field("screenshot", &self.screenshot.as_ref().map(|s| format!("[Screenshot Data: {} chars]", s.len())))
            .field("value", &self.value)
            .field("state", &self.state)
            .field("help_text", &self.help_text)
            .finish()
    }
}