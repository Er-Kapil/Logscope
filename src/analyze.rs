use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum LogLevels {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LogLevels {
    fn parse_level(value: &str) -> Option<Self> {
        match value {
            "TRACE" => Some(Self::TRACE),
            "DEBUG" => Some(Self::DEBUG),
            "INFO" => Some(Self::INFO),
            "WARN" => Some(Self::WARN),
            "ERROR" => Some(Self::ERROR),
            _ => None,
        }
    }
}

impl fmt::Display for LogLevels {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevels::TRACE => write!(f, "TRACE"),
            LogLevels::DEBUG => write!(f, "DEBUG"),
            LogLevels::INFO => write!(f, "INFO"),
            LogLevels::WARN => write!(f, "WARN"),
            LogLevels::ERROR => write!(f, "ERROR"),
        }
    }
}
struct Line<'a> {
    date_time: &'a str,
    level: LogLevels,
    content: &'a str,
}

fn is_valid_level(level: &str) -> bool {
    matches!(level, "TRACE" | "DEBUG" | "INFO" | "WARN" | "ERROR")
}

fn validate_line<'a>(line: &'a str) -> Option<Line<'a>> {
    let parts: Vec<&str> = line.splitn(4, ' ').collect();
    if parts.len() != 4 {
        return None;
    }
    let datetime = format!("{} {}", parts[0], parts[1]);
    if NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S,%3f").is_err() {
        return None;
    }
    if !is_valid_level(parts[2]) {
        return None;
    }
    return Some(Line {
        date_time: parts[0],
        level: LogLevels::parse_level(parts[2])?,
        content: parts[3],
    });
}

fn specifications(lines: &Vec<Line>) -> HashMap<LogLevels, i32> {
    let mut specs = HashMap::new();
    for line in lines {
        *specs.entry(line.level).or_default() += 1;
    }
    return specs;
}

pub fn analyser(filepath: &PathBuf) -> Result<(usize, HashMap<LogLevels, i32>), String> {
    let file_content: String = fs::read_to_string(filepath)
        .expect(&"Something went wrong in opening the file".to_string());
    let lines: Vec<Line> = file_content
        .lines()
        .filter_map(|line| validate_line(line))
        .collect();
    let total_entries = lines.len();
    let specs = specifications(&lines);
    return Ok((total_entries, specs));
}
