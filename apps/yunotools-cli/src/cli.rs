use yunotools_core::LogLevel;

use crate::option_catalog::{DEBUG_FLAGS, HELP_FLAGS, QUIET_FLAGS, SEARCH_FLAGS, VERSION_FLAGS};

pub enum CliAction {
    ShowHelp,
    ShowVersion,
    SearchModules { query: Option<String> },
    DispatchModule,
}

pub fn resolve_cli_action(raw_args: &[String], is_module_selected: bool) -> CliAction {
    if raw_args.len() == 1 || (has_flag(raw_args, HELP_FLAGS) && !is_module_selected) {
        return CliAction::ShowHelp;
    }

    if has_flag(raw_args, VERSION_FLAGS) {
        return CliAction::ShowVersion;
    }

    if let Some(query) = parse_search_query(raw_args) {
        return CliAction::SearchModules { query };
    }

    CliAction::DispatchModule
}

pub fn resolve_log_level(raw_args: &[String]) -> Option<LogLevel> {
    if has_flag(raw_args, DEBUG_FLAGS) {
        return Some(LogLevel::Debug);
    }

    if has_flag(raw_args, QUIET_FLAGS) {
        // Quiet vẫn giữ lại lỗi để CLI không thất bại trong im lặng.
        return Some(LogLevel::Error);
    }

    None
}

fn parse_search_query(raw_args: &[String]) -> Option<Option<String>> {
    for (arg_index, arg) in raw_args.iter().enumerate() {
        if SEARCH_FLAGS.contains(&arg.as_str()) {
            let query = raw_args
                .get(arg_index + 1)
                .filter(|value| !value.starts_with('-'))
                .cloned();

            return Some(query);
        }

        if let Some(query) = arg.strip_prefix("--search=") {
            return Some((!query.trim().is_empty()).then(|| query.to_owned()));
        }
    }

    None
}

fn has_flag(raw_args: &[String], expected_flags: &[&str]) -> bool {
    raw_args
        .iter()
        .any(|arg| expected_flags.contains(&arg.as_str()))
}
