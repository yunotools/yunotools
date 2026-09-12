use crate::binary::is_binary;
use crate::domain::TextFile;
use crate::ignore::IgnoreEngine;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Scanner;

impl Scanner {
    pub fn scan(root: &Path) -> Vec<TextFile> {
        let ignore = IgnoreEngine::new(root);

        let mut result = Vec::new();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            if ignore.ignored(path) {
                continue;
            }

            if is_binary(path) {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(path) {
                result.push(TextFile {
                    path: PathBuf::from(path),
                    content,
                })
            }
        }

        result
    }
}
