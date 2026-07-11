use super::metadata::{collect_metadata, print_metadata};
use std::path::Path;
use walkdir::WalkDir;

const IGNORED: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "__pycache__",
];

fn is_ignored(path: &Path) -> bool {
    path.components()
        .any(|c| IGNORED.contains(&c.as_os_str().to_string_lossy().as_ref()))
}

pub fn traverse_dir() {
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_ignored(e.path()))
    {
        let path = entry.path();

        match collect_metadata(path) {
            Ok(metadata) => print_metadata(path, &metadata),
            Err(e) => eprintln!("Error reading metadata for {}: {}", path.display(), e),
        }
    }
}
