use super::file::FileMetadata;
/*
Repository model for storing repository information
*/

pub struct Repository {
    pub name: String,
    pub root: String,
    pub git_enabled: bool,
    pub files: Vec<FileMetadata>,
}