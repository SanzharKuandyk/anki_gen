use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anki_gen")]
#[command(about = "Generate Anki flashcards using a local LLM (Ollama)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Ollama model name
    #[arg(long, default_value = "llama3", global = true)]
    pub model: String,

    /// Ollama API URL
    #[arg(long, default_value = "http://localhost:11434", global = true)]
    pub ollama_url: String,

    /// AnkiConnect URL
    #[arg(long, default_value = "http://localhost:8765", global = true)]
    pub anki_url: String,

    /// Anki deck name
    #[arg(long, short)]
    pub deck: Option<String>,

    /// Anki note type
    #[arg(long, short, default_value = "Kiku")]
    pub note_type: Option<String>,

    /// Card fields (comma-separated)
    #[arg(long, short, value_delimiter = ',')]
    pub fields: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check connectivity to Ollama and AnkiConnect
    Check,
    /// Generate a single card from a description
    Generate {
        /// Description of the card to generate
        description: String,
    },
    /// Generate the next card in a sequence (auto-skips already generated)
    Next {
        /// Category/topic description (e.g., "JLPT N3 grammar points")
        description: String,
    },
    /// Generate cards from a list (comma-separated or @filename)
    Batch {
        /// Comma-separated list of items, or @filename to read from file
        items: String,
    },
}
