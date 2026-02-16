use std::fs;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::types::StoredHistory;

pub struct FileStorage {
    path: PathBuf,
}

impl FileStorage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load_history(&self) -> Result<StoredHistory, AppError> {
        if !self.path.exists() {
            return Ok(StoredHistory::default());
        }
        let data = fs::read_to_string(&self.path)?;
        if data.trim().is_empty() {
            return Ok(StoredHistory::default());
        }
        let history: StoredHistory = serde_json::from_str(&data)?;
        Ok(history)
    }

    pub fn save_history(&self, history: &StoredHistory) -> Result<(), AppError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let data = serde_json::to_string_pretty(history)?;
        fs::write(&self.path, data)?;
        Ok(())
    }
}
