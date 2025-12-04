use serde::{Deserialize, Serialize};

/// Task status type (TODO or DONE)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskType {
    Todo,
    Done,
}

impl TaskType {
    /// Parse task type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "TODO" => Some(TaskType::Todo),
            "DONE" => Some(TaskType::Done),
            _ => None,
        }
    }
}

/// Task priority (A is highest, C is lowest)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    A,
    B,
    C,
    Other(char),
}

impl Priority {
    /// Create priority from character
    pub fn from_char(c: char) -> Self {
        match c {
            'A' => Priority::A,
            'B' => Priority::B,
            'C' => Priority::C,
            _ => Priority::Other(c),
        }
    }

    /// Get numeric order for sorting (lower is higher priority)
    pub fn order(&self) -> u32 {
        match self {
            Priority::A => 0,
            Priority::B => 1,
            Priority::C => 2,
            Priority::Other(c) => (*c as u32).saturating_sub('A' as u32),
        }
    }
}

/// Extracted task from markdown file
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub file: String,
    pub line: u32,
    pub heading: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<TaskType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_end_time: Option<String>,
}

/// Maximum file size to process (10 MB)
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Maximum number of tasks to extract
pub const MAX_TASKS: usize = 10_000;
