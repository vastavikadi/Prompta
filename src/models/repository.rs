use super::file::File;
/*
Repository model for storing repository information
*/

pub struct Repository {
    pub name: String,
    pub root: String,
    pub git_enabled: bool,
    pub files: Vec<File>,
}
