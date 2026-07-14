// use std::path::Path;
use crate::models::file::ProjectIndex;

use super::ignore::is_ignored;
use super::metadata::{collect_metadata, index_metadata};
use walkdir::WalkDir;

/*
scan_project helps to add a record of already indexed files/dir
*/
pub fn scan_project(root: &str) -> Result<ProjectIndex, Box<dyn std::error::Error>> {
    let mut index = ProjectIndex::new();
    traverse_dir(root, &mut index)?;
    Ok(index)
}

/*
traverse_dir to traverse the project directory
*/

fn traverse_dir(input: &str, index: &mut ProjectIndex) -> Result<(), std::io::Error> {
    for entry in WalkDir::new(input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_ignored(e.path()))
    {
        let path = entry.path();

        match collect_metadata(path) {
            Ok(metadata) => match index_metadata(path, &metadata, index) {
                Ok(metadata) => metadata,
                Err(e) => {
                    eprintln!("Error indexing metadata for {}: {}", path.display(), e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to index metadata for {}: {}", path.display(), e),
                    )
                    .into());
                }
            },
            Err(e) => {
                eprintln!("Error reading metadata for {}: {}", path.display(), e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read metadata for {}: {}", path.display(), e),
                )
                .into());
            }
        };
    }
    Ok(())
}
