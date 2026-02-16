mod anki_client;
mod cli;
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
use engine::Engine;
use model_client::OllamaClient;
use storage::FileStorage;
use types::CardRequest;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let model = OllamaClient::new(cli.ollama_url, cli.model);
    let anki = AnkiConnectClient::new(cli.anki_url);

    if matches!(cli.command, Commands::Check) {
        run_check(&model, &anki).await;
        return;
    }

    let deck = cli.deck.unwrap_or_else(|| {
        eprintln!("Error: --deck is required for this command");
        std::process::exit(1);
    });
    let note_type = cli.note_type.unwrap_or_else(|| {
        eprintln!("Error: --note-type is required for this command");
        std::process::exit(1);
    });
    if cli.fields.is_empty() {
        eprintln!("Error: --fields is required for this command");
        std::process::exit(1);
    }

    let storage = FileStorage::new(PathBuf::from("storage/used_grammar.json"));
    let engine = Engine::new(model, anki, storage);

    let result = match cli.command {
        Commands::Check => unreachable!(),
        Commands::Generate { description } => {
            let req = CardRequest {
                description,
                fields: cli.fields,
                note_type,
                deck,
            };
            engine.generate(&req).await
        }
        Commands::Next { description } => {
            let req = CardRequest {
                description,
                fields: cli.fields,
                note_type,
                deck,
            };
            engine.next(&req).await
        }
        Commands::Batch { items } => {
            let item_list = parse_items(&items);
            let req = CardRequest {
                description: String::new(),
                fields: cli.fields,
                note_type,
                deck,
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
    if let Some(path) = input.strip_prefix('@') {
        match std::fs::read_to_string(path) {
            Ok(content) => content
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect(),
            Err(e) => {
                eprintln!("Error reading file '{}': {}", path, e);
                std::process::exit(1);
            }
        }
    } else {
        input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
