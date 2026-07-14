use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/*
File model for storing file information
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub extension: String,
    // pub language: String,
    pub size: u64,
    pub hash: String,
    pub modified_at: DateTime<Utc>,
    pub indexed_at: DateTime<Utc>,
    pub is_dir: bool,
}

pub struct ProjectIndex {
    pub metadata: HashMap<PathBuf, FileMetadata>,
}

impl ProjectIndex {
    pub fn new() -> Self {
        ProjectIndex {
            metadata: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        id: Uuid,
        name: String,
        path: PathBuf,
        extension: String,
        size: u64,
        hash: String,
        modified_at: DateTime<Utc>,
        indexed_at: DateTime<Utc>,
        is_dir: bool,
    ) {
        self.metadata.insert(
            path.clone(),
            FileMetadata {
                id,
                name,
                path,
                extension,
                size,
                hash,
                modified_at,
                indexed_at,
                is_dir,
            },
        );
    }
}
