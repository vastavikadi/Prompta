mod indexer;
mod models;
mod utils;

use crate::indexer::walker::scan_project;

use std::io;

fn main() {
    println!("Starting the file indexing process...");
    println!(
        "Please enter the path to the project directory you want to index (or press Enter to use the current directory):"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input = input.trim();
    // if the user presses Enter without providing a path, use the current directory
    if input.is_empty() {
        println!("No input provided. Using the current directory.");
        let index = match scan_project(".") {
            Ok(index) => index,
            Err(e) => {
                eprintln!("Error scanning the project: {}", e);
                return;
            }
        };
        for (path, metadata) in &index.metadata {
            println!("{}: {} ({})", path.display(), metadata.id, metadata.size);
        }
    } else {
        println!("Indexing the provided directory: {}", input);
        let index = match scan_project(input) {
            Ok(index) => index,
            Err(e) => {
                eprintln!("Error scanning the project: {}", e);
                return;
            }
        };
        for (path, metadata) in &index.metadata {
            println!("{}: {} ({})", path.display(), metadata.id, metadata.size);
        }
    }
}
