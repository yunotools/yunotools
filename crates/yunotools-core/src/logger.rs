use chrono::Local;
use colored::Colorize;

fn timestamp() -> String {
    Local::now().format("%H:%M:%S").to_string()
}

pub fn info(message: &str) {
    println!("{} {} {}", timestamp(), "[INFO]".blue(), message);
}

pub fn success(message: &str) {
    println!("{} {} {}", timestamp().green(), "[OK]".green(), message);
}

pub fn warn(message: &str) {
    println!(
        "{} {} {}",
        timestamp().bright_black(),
        "[WARN]".yellow(),
        message
    );
}

pub fn error(message: &str) {
    println!(
        "{} {} {}",
        timestamp().bright_black(),
        "[ERROR]".red(),
        message
    );
}
