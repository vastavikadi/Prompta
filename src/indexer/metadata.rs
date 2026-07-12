use crate::utils::hash::hash_file;
use chrono::{DateTime, Utc};
use std::fs::Metadata;
use std::path::Path;

/*
Collect metadata for a given file or directory path.
*/

pub fn collect_metadata(path: &Path) -> Result<Metadata, std::io::Error> {
    std::fs::metadata(path)
}

pub fn print_metadata(path: &Path, metadata: &Metadata) {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let extension = path.extension().unwrap_or_default().to_string_lossy();
    let is_dir = metadata.is_dir();
    let size = metadata.len();
    let modified = metadata.modified().ok().map(|t| DateTime::<Utc>::from(t));
    let type_str = if is_dir { "DIR" } else { "FILE" };
    let hash = if !is_dir {
        match hash_file(path) {
            Ok(h) => h,
            Err(_) => "Error hashing file".to_string(),
        }
    } else {
        "N/A".to_string()
    };

    println!(
        "{:6} | {} | {} |{:>10} bytes | {} | {}  | {}",
        type_str,
        name,
        extension,
        size,
        path.display(),
        modified.map_or("N/A".to_string(), |t| { format!("{:?}", t) }),
        hash
    );
}
