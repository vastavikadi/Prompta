use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/*
File model for storing file information
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
    // pub language: String,
    pub size: u64,
    pub hash: String,
    pub modified_at: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
}
