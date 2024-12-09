use serde::{Serialize, Deserialize};
use super::element_info::ElementInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Guide {
    pub title: String,
    pub description: String,
    pub steps: Vec<GuideStep>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuideStep {
    pub sequence: i32,
    pub element: ElementInfo,
    pub step_description: String,
}