use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("AnkiConnect error: {0}")]
    Anki(String),

    #[error("Storage error: {0}")]
    Storage(#[from] std::io::Error),

    #[error("Model error: {0}")]
    Model(String),
}
