use std::path::Path;

/*
List of the directories to skip during the project indexing process
*/

const IGNORED: &[&str] = &[
    ".git",
    //for js based projects
    "node_modules",
    //nextjs based projects
    ".next",
    //rust based projects
    "target",
    //python based projects
    "dist",
    "build",
    "__pycache__",
    //java based projects
    "bin",
    //c based projects
    "obj",
    //go based projects
    "vendor",
];

pub fn is_ignored(path: &Path) -> bool {
    path.components()
        .any(|c| IGNORED.contains(&c.as_os_str().to_string_lossy().as_ref()))
}
