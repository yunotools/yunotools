use ignore::WalkBuilder;
use ignore::gitignore::*;
use std::path::Path;
use yunotools_core::logger;

// Những file ignore sử dụng cú pháp tương thích với gitignore
// `.gitignore` và `.ignore` được crate `ignore` hỗ trợ sẵn,
// nên không cần đặt trong danh sách này
pub const CUSTOM_IGNORE_FILES: &[&str] = &[
    ".yunotools-ignore",
    ".copyastignore",
    ".dockerignore",
    ".helmignore",
    ".npmignore",
    ".eslintignore",
    ".prettierignore",
    ".stylelintignore",
    ".vercelignore",
    ".gcloudignore",
    ".cfignore",
    ".slugignore",
    ".terraformignore",
    ".azureignore",
    ".funcignore",
    ".eleventyignore",
    ".parcelignore",
    ".nodemonignore",
    ".nxignore",
    ".bazelignore",
    ".serverlessignore",
    ".cursorignore",
    ".aiderignore",
    ".codeiumignore",
    ".copilotignore",
    ".fdignore",
    ".rgignore",
];

pub struct IgnoreEngine {
    matcher: Gitignore,
}

impl IgnoreEngine {
    // Function này chỉ đọc các file ignore ở folder root
    pub fn new(root: &Path) -> Self {
        let mut builder = GitignoreBuilder::new(root);

        let standard_ignore_files = [".gitignore", ".ignore"];

        let ignore_files_names = standard_ignore_files
            .into_iter()
            .chain(CUSTOM_IGNORE_FILES.iter().copied());

        for name in ignore_files_names {
            let ignore_path = root.join(name);

            if !ignore_path.is_file() {
                continue;
            }

            // cải thiện từ builder.build().unwrap()
            // không gây panic toàn bộ chương trình khi bị lỗi
            if let Some(error) = builder.add(&ignore_path) {
                logger::warn(&format!(
                    "Cannot read ignore file {}: {}",
                    ignore_path.display(),
                    error,
                ));
            }
        }

        let matcher = builder.build().unwrap_or_else(|error| {
            logger::warn(&format!("Cannot build ignore matcher: {}", error,));

            Gitignore::empty()
        });

        Self { matcher }
    }

    // Kiểm tra một path có bị ignore hay không.
    pub fn ignored(&self, path: &Path) -> bool {
        self.matcher.matched(path, path.is_dir()).is_ignore()
    }

    // Cấu hình WalkBuilder để hỗ trợ:
    // - ignore file nằm trong các folder con
    // - global Git ignore
    // - `.git/info/exclude`
    // - file và folder ẩn
    // - các custom ignore file phía trên
    pub fn configure_walker(builder: &mut WalkBuilder, respect_ignore: bool, include_hidden: bool) {
        builder
            // Trong WalkBuilder:
            // - hidden(true) nghĩa là bỏ qua file ẩn
            // - hidden(false) nghĩa là không bỏ qua file ẩn
            .hidden(!include_hidden)
            .parents(respect_ignore)
            .ignore(respect_ignore)
            .git_ignore(respect_ignore)
            .git_global(respect_ignore)
            .git_exclude(respect_ignore)
            // Cho phép .gitignore hoạt động ngay cả khi folder không có directory .git
            .require_git(false);

        if respect_ignore {
            for name in CUSTOM_IGNORE_FILES {
                builder.add_custom_ignore_filename(name);
            }
        }
    }
}
