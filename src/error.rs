use std::fmt;
use std::io;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    InvalidDirectory(String),
    InvalidGlob(String),
    InvalidDate(String),
    InvalidTimezone(String),
    DateRange(String),
    Serialization(String),
    Regex(String),
    Walk(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::InvalidDirectory(path) => write!(f, "Invalid directory: {path}"),
            AppError::InvalidGlob(pattern) => write!(f, "Invalid glob pattern: {pattern}"),
            AppError::InvalidDate(msg) => write!(f, "Invalid date: {msg}"),
            AppError::InvalidTimezone(tz) => write!(f, "Invalid timezone: {tz}"),
            AppError::DateRange(msg) => write!(f, "Invalid date range: {msg}"),
            AppError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
            AppError::Regex(msg) => write!(f, "Regex error: {msg}"),
            AppError::Walk(msg) => write!(f, "Walk error: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

impl From<ignore::Error> for AppError {
    fn from(err: ignore::Error) -> Self {
        AppError::Walk(err.to_string())
    }
}
