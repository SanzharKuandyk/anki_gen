use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anki_gen")]
#[command(about = "Generate Anki flashcards using a local LLM (Ollama)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Ollama model name
    #[arg(long, global = true)]
    pub model: Option<String>,

    /// Ollama API URL
    #[arg(long, global = true)]
    pub ollama_url: Option<String>,

    /// AnkiConnect URL
    #[arg(long, global = true)]
    pub anki_url: Option<String>,

    /// Anki deck name
    #[arg(long, short)]
    pub deck: Option<String>,

    /// Anki note type
    #[arg(long, short)]
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
    /// Generate example configuration file
    Config {
        /// Output format (yaml or json)
        #[arg(long, short, default_value = "yaml")]
        format: String,
    },
}
