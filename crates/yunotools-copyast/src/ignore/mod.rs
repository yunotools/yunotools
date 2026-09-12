mod generator;
mod template;

pub use generator::IgnoreGenerator;
pub use template::IgnoreTemplate;

use ::ignore::WalkBuilder;

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
    ".nowignore",
    ".netlifyignore",
    ".gcloudignore",
    ".cfignore",
    ".cloudflareignore",
    ".wranglerignore",
    ".slugignore",
    ".terraformignore",
    ".terragruntignore",
    ".azureignore",
    ".funcignore",
    ".eleventyignore",
    ".parcelignore",
    ".nodemonignore",
    ".nxignore",
    ".bazelignore",
    ".bazeliskignore",
    ".serverlessignore",
    ".sstignore",
    ".cursorignore",
    ".aiderignore",
    ".codeiumignore",
    ".copilotignore",
    ".continueignore",
    ".windsurfignore",
    ".clineignore",
    ".rooignore",
    ".geminiignore",
    ".fdignore",
    ".rgignore",
];

// Cấu hình WalkBuilder để hỗ trợ ignore file trong toàn bộ cây thư mục.
pub(crate) fn configure_walker(
    builder: &mut WalkBuilder,
    respect_ignore: bool,
    include_hidden: bool,
    additional_ignore_files: &[String],
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

        for name in additional_ignore_files {
            if !name.trim().is_empty() {
                builder.add_custom_ignore_filename(name);
            }
        }
    }
}
