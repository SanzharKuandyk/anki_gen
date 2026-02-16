use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::errors::AppError;
use crate::types::CardFields;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: serde_json::Value,
}

#[derive(Deserialize)]
struct StreamChunk {
    response: String,
    done: bool,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelEntry>,
}

#[derive(Deserialize)]
struct ModelEntry {
    name: String,
}

pub struct OllamaClient {
    base_url: String,
    model: String,
    client: reqwest::Client,
}

impl OllamaClient {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            base_url,
            model,
            client: reqwest::Client::new(),
        }
    }

    pub async fn ping(&self) -> Result<Vec<String>, AppError> {
        let url = format!("{}/api/tags", self.base_url);
        let resp = self.client.get(&url).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Model(format!(
                "Ollama returned status {}",
                resp.status()
            )));
        }

        let tags: TagsResponse = resp.json().await?;
        Ok(tags.models.into_iter().map(|m| m.name).collect())
    }

    pub fn model_name(&self) -> &str {
        &self.model
    }

    /// Build a JSON schema that enforces all fields are present and non-empty strings.
    fn build_schema(fields: &[String]) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        for field in fields {
            properties.insert(
                field.clone(),
                json!({
                    "type": "string",
                    "minLength": 1
                }),
            );
        }

        json!({
            "type": "object",
            "properties": properties,
            "required": fields,
            "additionalProperties": false
        })
    }

    pub async fn generate(&self, prompt: &str, fields: &[String]) -> Result<CardFields, AppError> {
        let schema = Self::build_schema(fields);

        let req = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: true,
            format: schema,
        };

        let url = format!("{}/api/generate", self.base_url);
        let mut resp = self.client.post(&url).json(&req).send().await?;

        if !resp.status().is_success() {
            return Err(AppError::Model(format!(
                "Ollama returned status {}",
                resp.status()
            )));
        }

        let mut buffer = String::new();
        let mut full_response = String::new();

        while let Some(chunk) = resp.chunk().await? {
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].to_string();
                buffer = buffer[pos + 1..].to_string();

                if line.trim().is_empty() {
                    continue;
                }

                if let Ok(parsed) = serde_json::from_str::<StreamChunk>(&line) {
                    print!("{}", parsed.response);
                    io::stdout().flush().ok();
                    full_response.push_str(&parsed.response);

                    if parsed.done {
                        println!();
                    }
                }
            }
        }

        // Handle any remaining data in buffer
        if !buffer.trim().is_empty() {
            if let Ok(parsed) = serde_json::from_str::<StreamChunk>(&buffer) {
                full_response.push_str(&parsed.response);
            }
        }

        let raw: CardFields = serde_json::from_str(&full_response)?;

        // Trim whitespace from keys and values
        let fields: CardFields = raw
            .into_iter()
            .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
            .collect();

        Ok(fields)
    }
}
