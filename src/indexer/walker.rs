use super::ignore::is_ignored;
use super::metadata::{collect_metadata, print_metadata};
use walkdir::WalkDir;

/*
Traverse fn for the project directory
*/

pub fn traverse_dir(input: &str) {
    for entry in WalkDir::new(input)
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
