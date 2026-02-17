use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,

    #[serde(default = "default_anki_url")]
    pub anki_url: String,

    #[serde(default)]
    pub deck: Option<String>,

    #[serde(default = "default_note_type")]
    pub note_type: String,

    #[serde(default)]
    pub fields: Vec<String>,

    #[serde(default = "default_storage_path")]
    pub storage_path: String,

    #[serde(default = "default_optional_fields")]
    pub optional_fields: bool,
}

// Default value functions
fn default_model() -> String {
    "llama3".to_string()
}

fn default_ollama_url() -> String {
    "http://localhost:11434".to_string()
}

fn default_anki_url() -> String {
    "http://localhost:8765".to_string()
}

fn default_note_type() -> String {
    "Kiku".to_string()
}

fn default_storage_path() -> String {
    "storage/used_grammar.json".to_string()
}

fn default_optional_fields() -> bool {
    false
}

impl Default for Config {
    fn default() -> Self {
        Config {
            model: default_model(),
            ollama_url: default_ollama_url(),
            anki_url: default_anki_url(),
            deck: None,
            note_type: default_note_type(),
            fields: Vec::new(),
            storage_path: default_storage_path(),
            optional_fields: default_optional_fields(),
        }
    }
}

impl Config {
    /// Load config with priority: CLI args > config file > defaults
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // Determine format by extension
        let config = if path.extension().and_then(|s| s.to_str()) == Some("yaml")
            || path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse YAML config: {}", e))?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON config: {}", e))?
        };

        Ok(config)
    }

    /// Try to load config from default locations, fallback to defaults if not found
    pub fn load_or_default() -> Self {
        // Try loading from common config file locations
        let config_paths = [
            "config.yaml",
            "config.yml",
            "config.json",
            ".anki_gen.yaml",
            ".anki_gen.json",
        ];

        for path in &config_paths {
            if Path::new(path).exists() {
                match Self::load_from_file(path) {
                    Ok(config) => {
                        eprintln!("✓ Loaded config from: {}", path);
                        return config;
                    }
                    Err(e) => {
                        eprintln!("⚠ Warning: {}", e);
                    }
                }
            }
        }

        // No config file found, use defaults
        Config::default()
    }

    /// Merge CLI overrides into config (CLI args take priority)
    pub fn merge_cli_overrides(&mut self, cli: &crate::cli::Cli) {
        // Only override if CLI arg was explicitly provided

        if let Some(ref model) = cli.model {
            self.model = model.clone();
        }

        if let Some(ref ollama_url) = cli.ollama_url {
            self.ollama_url = ollama_url.clone();
        }

        if let Some(ref anki_url) = cli.anki_url {
            self.anki_url = anki_url.clone();
        }

        if let Some(ref deck) = cli.deck {
            self.deck = Some(deck.clone());
        }

        if let Some(ref note_type) = cli.note_type {
            self.note_type = note_type.clone();
        }

        if !cli.fields.is_empty() {
            self.fields = cli.fields.clone();
        }

        // CLI flag always overrides if present (default is false)
        if cli.optional_fields {
            self.optional_fields = true;
        }
    }

    /// Generate example config files
    pub fn generate_example_yaml() -> String {
        let config = Config::default();
        serde_yaml::to_string(&config).unwrap_or_default()
    }

    pub fn generate_example_json() -> String {
        let config = Config::default();
        serde_json::to_string_pretty(&config).unwrap_or_default()
    }
}
