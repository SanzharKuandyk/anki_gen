use strsim::levenshtein;

use crate::anki_client::AnkiConnectClient;
use crate::errors::AppError;
use crate::model_client::OllamaClient;
use crate::prompt_builder::PromptBuilder;
use crate::storage::FileStorage;
use crate::types::{CardFields, CardRequest};

const MAX_EDIT_DISTANCE: usize = 2;

pub struct Engine {
    model: OllamaClient,
    anki: AnkiConnectClient,
    storage: FileStorage,
}

impl Engine {
    pub fn new(model: OllamaClient, anki: AnkiConnectClient, storage: FileStorage) -> Self {
        Self {
            model,
            anki,
            storage,
        }
    }

    /// Remap model output keys to match expected field names using fuzzy matching.
    fn fix_field_names(fields: CardFields, expected: &[String]) -> CardFields {
        let mut result = CardFields::new();

        for (key, value) in fields {
            if expected.iter().any(|e| e == &key) {
                result.insert(key, value);
            } else {
                let best = expected
                    .iter()
                    .filter(|e| !result.contains_key(e.as_str()))
                    .map(|e| (e, levenshtein(&key, e)))
                    .min_by_key(|(_, dist)| *dist);

                if let Some((expected_name, dist)) = best {
                    if dist <= MAX_EDIT_DISTANCE {
                        eprintln!(
                            "  Fuzzy fix: '{}' -> '{}' (edit distance {})",
                            key, expected_name, dist
                        );
                        result.insert(expected_name.clone(), value);
                    } else {
                        result.insert(key, value);
                    }
                } else {
                    result.insert(key, value);
                }
            }
        }

        result
    }

    fn validate_fields(
        fields: &CardFields,
        expected: &[String],
        optional: bool,
    ) -> Result<(), AppError> {
        if optional {
            // In optional mode, just check that at least one field has content
            let has_content = fields.values().any(|v| !v.trim().is_empty());
            if !has_content {
                return Err(AppError::Model(
                    "Model returned no content in any field".into(),
                ));
            }
        } else {
            // Strict mode: all expected fields must be present and non-empty
            let missing: Vec<&String> = expected
                .iter()
                .filter(|f| !fields.contains_key(f.as_str()))
                .collect();

            if !missing.is_empty() {
                return Err(AppError::Model(format!(
                    "Model response missing fields: {}. Got: {}",
                    missing
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                    fields.keys().cloned().collect::<Vec<_>>().join(", "),
                )));
            }

            let all_empty = expected
                .iter()
                .all(|f| fields.get(f.as_str()).map_or(true, |v| v.is_empty()));
            if all_empty {
                return Err(AppError::Model("Model returned all empty fields".into()));
            }
        }

        Ok(())
    }

    /// Validate Anki config. Returns all field names of the note type.
    async fn preflight(&self, req: &CardRequest) -> Result<Vec<String>, AppError> {
        println!("Checking Anki configuration...");
        let all_fields = self
            .anki
            .preflight(&req.deck, &req.note_type, &req.fields)
            .await?;
        println!(
            "  Deck: '{}' OK\n  Note type: '{}' OK\n  Fields: {:?} OK\n  Note type has {} total fields: {}",
            req.deck,
            req.note_type,
            req.fields,
            all_fields.len(),
            all_fields.join(", "),
        );
        Ok(all_fields)
    }

    pub async fn generate(&self, req: &CardRequest) -> Result<(), AppError> {
        let all_fields = self.preflight(req).await?;
        let prompt = PromptBuilder::build(req);
        println!("Generating card for: {}", req.description);

        let fields = self.model.generate(&prompt, &req.fields).await?;
        let fields = Self::fix_field_names(fields, &req.fields);
        Self::validate_fields(&fields, &req.fields, req.optional_fields)?;
        println!("Generated fields: {:?}", fields);

        self.anki
            .add_note(&fields, &req.note_type, &req.deck, &all_fields)
            .await?;
        println!("Card added to Anki!");

        let mut history = self.storage.load_history()?;
        history.used_items.push(req.description.clone());
        self.storage.save_history(&history)?;

        Ok(())
    }

    pub async fn next(&self, req: &CardRequest) -> Result<(), AppError> {
        let all_fields = self.preflight(req).await?;
        let history = self.storage.load_history()?;
        let prompt = PromptBuilder::build_next(req, &history.used_items);

        println!(
            "Generating next card (already have {} items)",
            history.used_items.len()
        );

        let fields = self.model.generate(&prompt, &req.fields).await?;
        let fields = Self::fix_field_names(fields, &req.fields);
        Self::validate_fields(&fields, &req.fields, req.optional_fields)?;
        println!("Generated fields: {:?}", fields);

        self.anki
            .add_note(&fields, &req.note_type, &req.deck, &all_fields)
            .await?;
        println!("Card added to Anki!");

        let item_name = req
            .fields
            .first()
            .and_then(|key| fields.get(key))
            .cloned()
            .unwrap_or_else(|| req.description.clone());

        let mut history = history;
        history.used_items.push(item_name);
        self.storage.save_history(&history)?;

        Ok(())
    }

    pub async fn batch(&self, req: &CardRequest, items: &[String]) -> Result<(), AppError> {
        let all_fields = self.preflight(req).await?;
        let mut history = self.storage.load_history()?;
        let total = items.len();

        let mut succeeded = 0;
        let mut failed = 0;
        let mut errors: Vec<(String, String)> = Vec::new();

        for (i, item) in items.iter().enumerate() {
            println!("[{}/{}] Generating: {}", i + 1, total, item);

            let item_req = CardRequest {
                description: item.clone(),
                fields: req.fields.clone(),
                note_type: req.note_type.clone(),
                deck: req.deck.clone(),
                optional_fields: req.optional_fields,
            };

            let result = async {
                let prompt = PromptBuilder::build(&item_req);
                let fields = self.model.generate(&prompt, &req.fields).await?;
                let fields = Self::fix_field_names(fields, &req.fields);
                Self::validate_fields(&fields, &req.fields, req.optional_fields)?;

                self.anki
                    .add_note(&fields, &req.note_type, &req.deck, &all_fields)
                    .await?;

                Ok::<(), AppError>(())
            }
            .await;

            match result {
                Ok(_) => {
                    println!("  ✓ Added to Anki");
                    history.used_items.push(item.clone());
                    succeeded += 1;
                }
                Err(e) => {
                    println!("  ✗ Failed: {}", e);
                    errors.push((item.clone(), e.to_string()));
                    failed += 1;
                }
            }
        }

        self.storage.save_history(&history)?;

        println!("\nBatch complete: {} succeeded, {} failed out of {}", succeeded, failed, total);

        if !errors.is_empty() {
            println!("\nFailed items:");
            for (item, error) in &errors {
                println!("  - {}: {}", item, error);
            }
        }

        if succeeded == 0 {
            Err(AppError::Model("All batch items failed".into()))
        } else {
            Ok(())
        }
    }
}
