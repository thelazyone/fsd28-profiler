use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Modifier {
    pub id: String,
    pub points: u32,
    pub effects: Value, // Use serde_json::Value for dynamic content
}
