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
            format!("[{}] {} : {}", timestamp, colorize_level(level), message)
        }
        _ => line.to_string(),
    }
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

fn colorize_level(l: String) -> ColoredString {
    match level(l.as_str()) {
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
