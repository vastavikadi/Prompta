mod indexer;
mod models;
mod utils;

use crate::indexer::walker::traverse_dir;

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
        traverse_dir(".");
    } else {
        println!("Indexing the provided directory: {}", input);
        traverse_dir(input);
    }
}
