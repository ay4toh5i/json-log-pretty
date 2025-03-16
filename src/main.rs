use chrono::{Local, TimeZone};
use colored::{ColoredString, Colorize};
use serde_json::{Map, Value};
use std::io;

struct Keys<'a> {
    message: &'a str,
    level: &'a str,
    timestamp: &'a str,
}

const KEYS: Keys = Keys {
    message: "message",
    level: "level",
    timestamp: "timestamp",
};

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

struct Level<'a> {
    debug: &'a str,
    info: &'a str,
    warn: &'a str,
    error: &'a str,
    fatal: &'a str,
}

static LEVEL: Level = Level {
    debug: "DEBUG",
    info: "INFO",
    warn: "WARN",
    error: "ERROR",
    fatal: "FATAL",
};

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("{}", pretty(&line.unwrap()));
    }
}

fn pretty(line: &str) -> String {
    let v: Result<Value, _> = serde_json::from_str(line);

    match v {
        Ok(Value::Object(value)) => {
            let message = get_value_as_string(&value, KEYS.message);
            let level = get_value_as_string(&value, KEYS.level);
            let timestamp = get_value_as_string(&value, KEYS.timestamp);
            format!("[{}] {} : {}", format_timestamp(timestamp.as_str()), colorize_level(level.as_str()), message)
        }
        _ => line.to_string(),
    }
}

fn format_timestamp(timestamp: &str) -> String {
    let date_time = Local.timestamp_opt(timestamp.parse::<i64>().unwrap(), 0).unwrap();
    date_time.format("%H:%M:%S").to_string()
}

fn level(level: &str) -> LogLevel {
    if level == LEVEL.debug {
        LogLevel::Debug
    } else if level == LEVEL.info {
        LogLevel::Info
    } else if level == LEVEL.warn {
        LogLevel::Warn
    } else if level == LEVEL.error {
        LogLevel::Error
    } else if level == LEVEL.fatal {
        LogLevel::Fatal
    } else {
        LogLevel::Info
    }
}

fn colorize_level(l: &str) -> ColoredString {
    match level(l) {
        LogLevel::Debug => l.normal(),
        LogLevel::Info => l.green(),
        LogLevel::Warn => l.yellow(),
        LogLevel::Error => l.red(),
        LogLevel::Fatal => l.red().bold(),
    }
}

fn get_value_as_string(value: &Map<String, Value>, key: &str) -> String {
    match value.get(key) {
        Some(v) => v.as_str().unwrap_or("").to_string(),
        None => "".to_string(),
    }
}
