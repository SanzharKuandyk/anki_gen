use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// What the user asks for when generating a card.
pub struct CardRequest {
    pub description: String,
    pub fields: Vec<String>,
    pub note_type: String,
    pub deck: String,
}

/// The model's output — field name to field value.
pub type CardFields = HashMap<String, String>;

/// History of items already generated (persisted to disk).
#[derive(Serialize, Deserialize, Default)]
pub struct StoredHistory {
    pub used_items: Vec<String>,
}
