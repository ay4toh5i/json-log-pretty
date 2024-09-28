use std::io;
use colored::Colorize;
use serde_json::{Value, Map};

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

fn main() {
    println!("Hello, world!");

    let lines = io::stdin().lines();
    for line in lines {
        println!("{}: {}", "output".green(), pretty(&line.unwrap()));
    }
}

fn pretty(line: &str) -> String {
    let v: Result<Value, _> = serde_json::from_str(line);

    match v {
        Ok(Value::Object(value)) => {
            let message = get_value_as_string(&value, KEYS.message);
            let level = get_value_as_string(&value, KEYS.level);
            let timestamp = get_value_as_string(&value, KEYS.timestamp);
            format!("{}: {} - {}", timestamp, level.green(), message)
        }
        _ => line.to_string(),
    }
}

fn get_value_as_string(value: &Map<String, Value>, key: &str) -> String {
    match value.get(key) {
        Some(v) => v.as_str().unwrap_or("").to_string(),
        None => "".to_string(),
    }
}
