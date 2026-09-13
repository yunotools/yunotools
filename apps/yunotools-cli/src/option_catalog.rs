// Thông tin hiển thị và tra cứu của một tham số global.
pub struct GlobalOption {
    pub flags: &'static [&'static str],
    pub value_name: Option<&'static str>,
    pub description: &'static str,
}

pub const HELP_FLAGS: &[&str] = &["-h", "--help"];
pub const VERSION_FLAGS: &[&str] = &["-v", "--version"];
pub const SEARCH_FLAGS: &[&str] = &["-s", "--search"];
pub const DEBUG_FLAGS: &[&str] = &["-d", "--debug", "--verbose"];
pub const QUIET_FLAGS: &[&str] = &["-q", "--quiet"];

// Danh bạ tập trung của các tham số dùng chung cho toàn bộ ứng dụng.
pub const GLOBAL_OPTIONS: &[GlobalOption] = &[
    GlobalOption {
        flags: HELP_FLAGS,
        value_name: None,
        description: "Show this help",
    },
    GlobalOption {
        flags: VERSION_FLAGS,
        value_name: None,
        description: "Show application version",
    },
    GlobalOption {
        flags: SEARCH_FLAGS,
        value_name: Some("[QUERY]"),
        description: "Search modules; omit QUERY to list all",
    },
    GlobalOption {
        flags: DEBUG_FLAGS,
        value_name: None,
        description: "Enable debug logs",
    },
    GlobalOption {
        flags: QUIET_FLAGS,
        value_name: None,
        description: "Hide non-error logs",
    },
];
