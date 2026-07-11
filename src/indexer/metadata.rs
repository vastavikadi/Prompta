use chrono::{DateTime, Utc};
use std::fs::Metadata;
use std::path::Path;

pub fn collect_metadata(path: &Path) -> Result<Metadata, std::io::Error> {
    std::fs::metadata(path)
}

pub fn print_metadata(path: &Path, metadata: &Metadata) {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let is_dir = metadata.is_dir();
    let size = metadata.len();
    let modified = metadata.modified().ok().map(|t| DateTime::<Utc>::from(t));
    let type_str = if is_dir { "DIR" } else { "FILE" };

    println!(
        "{:6} | {} | {:>10} bytes | {} | {}",
        type_str,
        name,
        size,
        path.display(),
        modified.map_or("N/A".to_string(), |t| { format!("{:?}", t) })
    );
}
