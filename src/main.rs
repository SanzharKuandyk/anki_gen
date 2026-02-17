mod anki_client;
mod cli;
mod config;
mod engine;
mod errors;
mod model_client;
mod prompt_builder;
mod storage;
mod types;

use std::path::PathBuf;

use clap::Parser;

use anki_client::AnkiConnectClient;
use cli::{Cli, Commands};
use config::Config;
use engine::Engine;
use model_client::OllamaClient;
use storage::FileStorage;
use types::CardRequest;

#[tokio::main]
async fn main() {
    // Load config with priority: CLI args > config file > defaults
    let mut config = Config::load_or_default();
    let cli = Cli::parse();
    config.merge_cli_overrides(&cli);

    let model = OllamaClient::new(config.ollama_url.clone(), config.model.clone());
    let anki = AnkiConnectClient::new(config.anki_url.clone());

    // Handle commands that don't need full config
    match &cli.command {
        Commands::Check => {
            run_check(&model, &anki).await;
            return;
        }
        Commands::Config { format } => {
            generate_config_file(format);
            return;
        }
        _ => {}
    }

    let deck = config.deck.clone().unwrap_or_else(|| {
        eprintln!("Error: --deck is required for this command (set via CLI or config file)");
        std::process::exit(1);
    });
    let note_type = config.note_type.clone();

    // If fields not specified, auto-detect from note type
    let fields = if config.fields.is_empty() {
        eprintln!("No fields specified, auto-detecting from note type '{}'...", note_type);
        match anki.get_model_field_names(&note_type).await {
            Ok(all_fields) => {
                eprintln!("Auto-detected {} fields: {}", all_fields.len(), all_fields.join(", "));
                if config.optional_fields {
                    eprintln!("Using optional mode - model will fill only relevant fields");
                } else {
                    eprintln!("Using strict mode - model must fill all fields (consider --optional-fields)");
                }
                all_fields
            }
            Err(e) => {
                eprintln!("Error: Could not get fields for note type '{}': {}", note_type, e);
                eprintln!("Either specify --fields explicitly or ensure the note type exists in Anki");
                std::process::exit(1);
            }
        }
    } else {
        config.fields.clone()
    };

    let storage = FileStorage::new(PathBuf::from(&config.storage_path));
    let engine = Engine::new(model, anki, storage);

    let result = match cli.command {
        Commands::Check | Commands::Config { .. } => unreachable!(),
        Commands::Generate { description } => {
            let req = CardRequest {
                description,
                fields: fields.clone(),
                note_type,
                deck,
                optional_fields: config.optional_fields,
            };
            engine.generate(&req).await
        }
        Commands::Next { description } => {
            let req = CardRequest {
                description,
                fields: fields.clone(),
                note_type,
                deck,
                optional_fields: config.optional_fields,
            };
            engine.next(&req).await
        }
        Commands::Batch { items } => {
            let item_list = parse_items(&items);
            let req = CardRequest {
                description: String::new(),
                fields: fields.clone(),
                note_type,
                deck,
                optional_fields: config.optional_fields,
            };
            engine.batch(&req, &item_list).await
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run_check(model: &OllamaClient, anki: &AnkiConnectClient) {
    let mut ok = true;

    print!("Ollama ({})... ", model.model_name());
    match model.ping().await {
        Ok(models) => {
            if models.iter().any(|m| m.starts_with(model.model_name())) {
                println!("OK (model found)");
            } else {
                println!(
                    "WARNING: connected but model '{}' not found. Available: {}",
                    model.model_name(),
                    models.join(", ")
                );
            }
        }
        Err(e) => {
            println!("FAIL ({})", e);
            ok = false;
        }
    }

    print!("AnkiConnect... ");
    match anki.ping().await {
        Ok(version) => println!("OK (version {})", version),
        Err(e) => {
            println!("FAIL ({})", e);
            ok = false;
        }
    }

    if ok {
        println!("\nAll checks passed.");
    } else {
        println!("\nSome checks failed.");
        std::process::exit(1);
    }
}

fn parse_items(input: &str) -> Vec<String> {
    let items = if let Some(path) = input.strip_prefix('@') {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                let items: Vec<String> = content
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect();
                eprintln!("Loaded {} items from file '{}'", items.len(), path);
                items
            }
            Err(e) => {
                eprintln!("Error reading file '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        let items: Vec<String> = input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        eprintln!("Parsed {} items from input", items.len());
        items
    };

    if items.is_empty() {
        eprintln!("Warning: No items to process!");
    }

    items
}

fn generate_config_file(format: &str) {
    let (content, filename) = match format.to_lowercase().as_str() {
        "json" => (Config::generate_example_json(), "config.json"),
        "yaml" | "yml" => (Config::generate_example_yaml(), "config.yaml"),
        _ => {
            eprintln!("Error: Unknown format '{}'. Use 'yaml' or 'json'.", format);
            std::process::exit(1);
        }
    };

    println!("{}", content);
    println!("\n# To use this config, save it to '{}'", filename);
}
