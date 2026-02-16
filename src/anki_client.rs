use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::AppError;
use crate::types::CardFields;

#[derive(Serialize)]
struct AnkiRequest {
    action: String,
    version: u8,
    params: Value,
}

#[derive(Deserialize)]
struct AnkiResponse {
    #[allow(dead_code)]
    result: Option<Value>,
    error: Option<String>,
}

pub struct AnkiConnectClient {
    url: String,
    client: reqwest::Client,
}

impl AnkiConnectClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping(&self) -> Result<u64, AppError> {
        let anki_resp = self.request("version", serde_json::json!({})).await?;
        if let Some(err) = anki_resp.error {
            return Err(AppError::Anki(err));
        }
        let version = anki_resp.result.and_then(|v| v.as_u64()).unwrap_or(0);
        Ok(version)
    }

    async fn request(&self, action: &str, params: Value) -> Result<AnkiResponse, AppError> {
        let req = AnkiRequest {
            action: action.to_string(),
            version: 6,
            params,
        };
        let resp = self.client.post(&self.url).json(&req).send().await?;
        let anki_resp: AnkiResponse = resp.json().await?;
        Ok(anki_resp)
    }

    pub async fn get_deck_names(&self) -> Result<Vec<String>, AppError> {
        let anki_resp = self.request("deckNames", serde_json::json!({})).await?;
        if let Some(err) = anki_resp.error {
            return Err(AppError::Anki(err));
        }
        let names: Vec<String> = anki_resp
            .result
            .map(|v| serde_json::from_value(v).unwrap_or_default())
            .unwrap_or_default();
        Ok(names)
    }

    pub async fn get_model_names(&self) -> Result<Vec<String>, AppError> {
        let anki_resp = self.request("modelNames", serde_json::json!({})).await?;
        if let Some(err) = anki_resp.error {
            return Err(AppError::Anki(err));
        }
        let names: Vec<String> = anki_resp
            .result
            .map(|v| serde_json::from_value(v).unwrap_or_default())
            .unwrap_or_default();
        Ok(names)
    }

    pub async fn get_model_field_names(&self, model: &str) -> Result<Vec<String>, AppError> {
        let anki_resp = self
            .request(
                "modelFieldNames",
                serde_json::json!({ "modelName": model }),
            )
            .await?;
        if let Some(err) = anki_resp.error {
            return Err(AppError::Anki(err));
        }
        let names: Vec<String> = anki_resp
            .result
            .map(|v| serde_json::from_value(v).unwrap_or_default())
            .unwrap_or_default();
        Ok(names)
    }

    /// Validate deck, note type, and fields exist. Returns all fields of the note type.
    pub async fn preflight(
        &self,
        deck: &str,
        note_type: &str,
        fields: &[String],
    ) -> Result<Vec<String>, AppError> {
        // Check deck exists
        let decks = self.get_deck_names().await?;
        if !decks.iter().any(|d| d == deck) {
            return Err(AppError::Anki(format!(
                "Deck '{}' not found. Available: {}",
                deck,
                decks.join(", ")
            )));
        }

        // Check note type exists
        let models = self.get_model_names().await?;
        if !models.iter().any(|m| m == note_type) {
            return Err(AppError::Anki(format!(
                "Note type '{}' not found. Available: {}",
                note_type,
                models.join(", ")
            )));
        }

        // Check fields match
        let model_fields = self.get_model_field_names(note_type).await?;
        let missing: Vec<&String> = fields
            .iter()
            .filter(|f| !model_fields.contains(f))
            .collect();
        if !missing.is_empty() {
            return Err(AppError::Anki(format!(
                "Fields {:?} not found in note type '{}'. Available fields: {}",
                missing,
                note_type,
                model_fields.join(", ")
            )));
        }

        // Warn if sort field (first field) is not in the user's requested fields
        if let Some(sort_field) = model_fields.first() {
            if !fields.contains(sort_field) {
                eprintln!(
                    "  WARNING: Sort field '{}' is not in your --fields list. \
                     It will be left empty unless other fields cover it.",
                    sort_field
                );
            }
        }

        Ok(model_fields)
    }

    pub async fn add_note(
        &self,
        fields: &CardFields,
        note_type: &str,
        deck: &str,
        all_model_fields: &[String],
    ) -> Result<(), AppError> {
        // Build full fields map — every note type field present, empty if not provided
        let mut full_fields = serde_json::Map::new();
        for field_name in all_model_fields {
            let value = fields
                .get(field_name)
                .cloned()
                .unwrap_or_default();
            full_fields.insert(field_name.clone(), Value::String(value));
        }

        let note = serde_json::json!({
            "deckName": deck,
            "modelName": note_type,
            "fields": full_fields,
            "options": {
                "allowDuplicate": false
            }
        });

        let anki_resp = self
            .request("addNote", serde_json::json!({ "note": note }))
            .await?;

        if let Some(err) = anki_resp.error {
            return Err(AppError::Anki(err));
        }

        Ok(())
    }
}
