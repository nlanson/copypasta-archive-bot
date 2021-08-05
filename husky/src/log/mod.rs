#[allow(dead_code)]
use colored::*;

pub enum Level {
    Info,
    Warning,
    Error
}

pub fn print(level: Level, msg: &str) {
    match level {
        Level::Info => {
            println!("{} {}", "[INFO]".white().bold(), msg.white());
        },
        Level::Warning => {
            println!("{} {}", "[WARN]".yellow().bold(), msg.yellow());
        },
        Level::Error => {
            println!("{} {}", "[ERR]".red().bold(), msg.red());
        }
    }
}