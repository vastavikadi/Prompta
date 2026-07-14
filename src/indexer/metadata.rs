use crate::models::file::ProjectIndex;
use crate::utils::hash::hash_file;
use chrono::{DateTime, Utc};
use std::borrow::Cow;
use std::fs::Metadata;
use std::path::Path;
use uuid::Uuid;

/*
Collect metadata for a given file or directory path.
*/

pub fn collect_metadata(path: &Path) -> Result<Metadata, std::io::Error> {
    std::fs::metadata(path)
}

pub fn index_metadata(path: &Path, metadata: &Metadata, index: &mut ProjectIndex) -> Result<(), std::io::Error> {
    let name = match path.file_name().unwrap_or_default().to_string_lossy() {
        Cow::Borrowed(s) => s.to_string(),
        Cow::Owned(s) => s,
    };

    let is_dir = metadata.is_dir();
    let size = metadata.len();
    let modified = match metadata.modified().ok().map(|t| DateTime::<Utc>::from(t)) {
        Some(dt) => dt,
        None => Utc::now(),
    };
    // let type_str = if is_dir { "DIR" } else { "FILE" };

    if !is_dir {
        let extension = match path.extension().unwrap_or_default().to_string_lossy() {
            Cow::Borrowed(s) => s.to_string(),
            Cow::Owned(s) => s,
        };
        let hash = match hash_file(path) {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Error generating hash for the file: {e}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate hash"));
            }
        };
        let uuid = match get_uuid_for_file(&hash) {
            Ok(uuid) => uuid,
            Err(e) => {
                eprintln!("Error generating uuid for the file: {e}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate uuid"));
            }
        };
        index.add(
            uuid,
            name,
            path.to_path_buf(),
            extension,
            size,
            hash,
            modified,
            Utc::now(),
            is_dir,
        );
    } else {
        let uuid = match get_uuid_for_dir(path) {
            Ok(uuid) => uuid,
            Err(e) => {
                eprintln!("Error generating uuid for the directory: {e}");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate uuid"));
            }
        };
        index.add(
            uuid,
            name,
            path.to_path_buf(),
            String::new(),
            size,
            String::new(),
            modified,
            Utc::now(),
            is_dir,
        );
    }
    Ok(())
}

fn get_uuid_for_file(hash: &String) -> Result<Uuid, std::io::Error> {
    let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, hash.as_bytes());
    Ok(id)
}

fn get_uuid_for_dir(path: &Path) -> Result<Uuid, std::io::Error> {
    let path_str = path.to_string_lossy();
    let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, path_str.as_bytes());
    Ok(id)
}
