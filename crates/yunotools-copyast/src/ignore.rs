use ignore::gitignore::*;
use std::path::Path;

pub struct IgnoreEngine {
    matcher: Gitignore,
}

impl IgnoreEngine {
    pub fn new(root: &Path) -> Self {
        let mut builder = GitignoreBuilder::new(root);

        let files = [
            ".gitignore",
            ".dockerignore",
            ".helmignore",
            ".npmignore",
            ".eslintignore",
            ".prettierignore",
        ];

        for file in files {
            let path = root.join(file);
            if path.exists() {
                let _ = builder.add(path);
            }
        }

        Self {
            matcher: builder.build().unwrap(),
        }
    }

    pub fn ignored(&self, path: &Path) -> bool {
        self.matcher.matched(path, false).is_ignore()
    }
}
