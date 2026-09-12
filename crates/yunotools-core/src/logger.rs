use chrono::Local;
use colored::{Color, Colorize};
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Off = 4,
}

static MINIMUM_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);

pub fn init_from_env() {
    let Ok(value) = std::env::var("YUNTUNS_LOG") else {
        return;
    };

    let level = match value.trim().to_ascii_lowercase().as_str() {
        "debug" | "trace" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" | "warning" => LogLevel::Warn,
        "error" => LogLevel::Error,
        "off" | "none" => LogLevel::Off,
        _ => return,
    };

    set_level(level);
}

pub fn set_level(level: LogLevel) {
    MINIMUM_LEVEL.store(level as u8, Ordering::Relaxed);
}

fn timestamp() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn info(message: &str) {
    log(LogLevel::Info, "INFO", Color::Blue, message);
}

pub fn success(message: &str) {
    log(LogLevel::Info, " OK ", Color::Green, message);
}

pub fn warn(message: &str) {
    log(LogLevel::Warn, "WARN", Color::Yellow, message);
}

pub fn error(message: &str) {
    log(LogLevel::Error, "ERROR", Color::Red, message);
}

pub fn debug(message: &str) {
    log(LogLevel::Debug, "DEBUG", Color::BrightBlack, message);
}

fn log(level: LogLevel, label: &str, color: Color, message: &str) {
    if (level as u8) < MINIMUM_LEVEL.load(Ordering::Relaxed) {
        return;
    }

    eprintln!(
        "{} {} {}",
        timestamp().bright_black(),
        format!("[{label}]").color(color).bold(),
        message,
    );
}
