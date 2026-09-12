use std::path::PathBuf;

// Kích thước tối đa mặc định của 1 file: 10 MiB
pub const DEFAULT_MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

// Quyết định đường dẫn ghi trong header của mỗi file
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PathMode {
    // tự động
    //  - input tương đối -> Header tương đối
    //  - input tuyệt đối -> Header tuyệt đối
    #[default]
    Auto,

    Relative,
    Absolute,
}

// Loại tokenizer mà bộ ước lượng token mô phỏng
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TokenModel {
    Cl100k,

    #[default]
    O200k,

    Claude,
    Gemini,
}

impl TokenModel {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Cl100k => "cl100k",
            Self::O200k => "o200k",
            Self::Claude => "claude",
            Self::Gemini => "gemini",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CopyastConfig {
    // File hoặc folder cần đọc
    pub input: PathBuf,

    // File hoặc folder nhận kết quả
    pub output: PathBuf,

    // Cách hiển thị đường dẫn trong header
    pub path_mode: PathMode,

    // Model dùng để ước lượng token
    pub token_model: TokenModel,

    // Chỉ cập nhật output khi source thay đổi
    pub incremental: bool,

    // Chỉ giữ file đầu tiên nếu nội dung bị trùng
    pub deduplicate: bool,

    // Chỉ quét và báo cáo, không ghi output
    pub dry_run: bool,

    // Có sử dụng các file ignore hay không
    pub respect_ignore: bool,

    // Có quét các file/folder ẩn hay không
    pub include_hidden: bool,

    // Các tên ignore file bổ sung, ví dụ `.mycompanyignore`.
    pub additional_ignore_files: Vec<String>,

    pub max_file_size: u64,
}

impl CopyastConfig {
    pub fn new(input: impl Into<PathBuf>, output: impl Into<PathBuf>) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            path_mode: PathMode::Auto,
            token_model: TokenModel::O200k,
            incremental: false,
            deduplicate: false,
            dry_run: false,
            respect_ignore: true,
            include_hidden: false,
            additional_ignore_files: Vec::new(),
            max_file_size: DEFAULT_MAX_FILE_SIZE,
        }
    }

    pub fn with_path_mode(mut self, path_mode: PathMode) -> Self {
        self.path_mode = path_mode;
        self
    }

    pub fn with_token_model(mut self, token_model: TokenModel) -> Self {
        self.token_model = token_model;
        self
    }

    pub fn with_incremental(mut self, enabled: bool) -> Self {
        self.incremental = enabled;
        self
    }

    pub fn with_deduplication(mut self, enabled: bool) -> Self {
        self.deduplicate = enabled;
        self
    }

    pub fn with_dry_run(mut self, enabled: bool) -> Self {
        self.dry_run = enabled;
        self
    }

    pub fn with_ignore_files(mut self, enabled: bool) -> Self {
        self.respect_ignore = enabled;
        self
    }

    pub fn with_hidden_files(mut self, enabled: bool) -> Self {
        self.include_hidden = enabled;
        self
    }

    pub fn with_additional_ignore_files(mut self, file_names: Vec<String>) -> Self {
        self.additional_ignore_files = file_names;
        self
    }

    pub fn with_max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = max_file_size;
        self
    }
}
