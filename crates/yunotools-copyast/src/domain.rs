use std::path::PathBuf;

pub struct TextFile {
    pub path: PathBuf,
    pub content: String,
}

pub struct CopyastResult {
    pub copied: usize,
    pub skipped: usize,
}
