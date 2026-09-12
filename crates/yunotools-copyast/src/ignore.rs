use ignore::WalkBuilder;

// Những file ignore sử dụng cú pháp tương thích với gitignore
// `.gitignore` và `.ignore` được crate `ignore` hỗ trợ sẵn,
// nên không cần đặt trong danh sách này
pub(crate) const CUSTOM_IGNORE_FILES: &[&str] = &[
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

// Cấu hình WalkBuilder để hỗ trợ ignore file trong toàn bộ cây thư mục.
pub(crate) fn configure_walker(
    builder: &mut WalkBuilder,
    respect_ignore: bool,
    include_hidden: bool,
) {
    builder
        // hidden(true) nghĩa là bỏ qua file và folder ẩn.
        .hidden(!include_hidden)
        .parents(respect_ignore)
        .ignore(respect_ignore)
        .git_ignore(respect_ignore)
        .git_global(respect_ignore)
        .git_exclude(respect_ignore)
        // Cho phép .gitignore hoạt động ngoài Git repository.
        .require_git(false);

    if respect_ignore {
        for name in CUSTOM_IGNORE_FILES {
            builder.add_custom_ignore_filename(name);
        }
    }
}
