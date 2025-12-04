use chrono::{Datelike, NaiveDate, TimeZone};
use chrono_tz::Tz;

use crate::timestamp::{parse_org_timestamp, timestamp_in_range, timestamp_matches_date};
use crate::types::{Task, TaskType};

/// Filter tasks based on agenda mode
///
/// # Arguments
/// * `tasks` - Tasks to filter
/// * `mode` - Agenda mode: "day", "week", or "tasks"
/// * `date` - Optional date for "day" mode (YYYY-MM-DD)
/// * `from` - Optional start date for "week" mode (YYYY-MM-DD)
/// * `to` - Optional end date for "week" mode (YYYY-MM-DD)
/// * `tz` - Timezone string (e.g., "Europe/Moscow")
///
/// # Returns
/// Filtered and sorted tasks
///
/// # Errors
/// Returns error if timezone is invalid or date parsing fails
pub fn filter_agenda(
    mut tasks: Vec<Task>,
    mode: &str,
    date: Option<&str>,
    from: Option<&str>,
    to: Option<&str>,
    tz: &str,
) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let tz: Tz = tz
        .parse()
        .map_err(|_| format!("Invalid timezone: {tz}. Use IANA timezone names (e.g., 'Europe/Moscow', 'UTC')"))?;

    match mode {
        "day" => {
            let target_date = if let Some(date_str) = date {
                NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid date format '{date_str}': {e}. Use YYYY-MM-DD"))?
            } else {
                tz.from_utc_datetime(&chrono::Utc::now().naive_utc())
                    .date_naive()
            };
            tasks.retain(|t| task_matches_date(t, &target_date));
        }
        "week" => {
            let (start_date, end_date) = if let (Some(from_str), Some(to_str)) = (from, to) {
                let start = NaiveDate::parse_from_str(from_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid 'from' date '{from_str}': {e}. Use YYYY-MM-DD"))?;
                let end = NaiveDate::parse_from_str(to_str, "%Y-%m-%d")
                    .map_err(|e| format!("Invalid 'to' date '{to_str}': {e}. Use YYYY-MM-DD"))?;
                
                if start > end {
                    return Err(format!("Start date {from_str} is after end date {to_str}").into());
                }
                
                (start, end)
            } else {
                get_current_week(&tz)
            };
            tasks.retain(|t| task_in_range(t, &start_date, &end_date));
        }
        "tasks" => {
            tasks.retain(|t| matches!(t.task_type, Some(TaskType::Todo)));
            tasks.sort_by_key(|t| t.priority.as_ref().map(|p| p.order()).unwrap_or(999));
        }
        _ => {
            return Err(format!(
                "Invalid agenda mode '{mode}'. Valid modes: 'day', 'week', 'tasks'"
            )
            .into())
        }
    }
    Ok(tasks)
}

/// Get current week (Monday to Sunday) in the given timezone
fn get_current_week(tz: &Tz) -> (NaiveDate, NaiveDate) {
    let today = tz
        .from_utc_datetime(&chrono::Utc::now().naive_utc())
        .date_naive();
    let weekday = today.weekday();
    let days_from_monday = weekday.num_days_from_monday();
    let monday = today - chrono::Duration::days(days_from_monday as i64);
    let sunday = monday + chrono::Duration::days(6);
    (monday, sunday)
}

/// Check if task matches a specific date
fn task_matches_date(task: &Task, target_date: &NaiveDate) -> bool {
    if let Some(ref ts) = task.timestamp {
        if let Some(parsed) = parse_org_timestamp(ts, None) {
            return timestamp_matches_date(&parsed, target_date);
        }
    }
    false
}

/// Check if task falls within a date range
fn task_in_range(task: &Task, start: &NaiveDate, end: &NaiveDate) -> bool {
    if let Some(ref ts) = task.timestamp {
        if let Some(parsed) = parse_org_timestamp(ts, None) {
            return timestamp_in_range(&parsed, start, end);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Priority;

    #[test]
    fn test_filter_tasks_mode() {
        let tasks = vec![
            Task {
                file: "test.md".to_string(),
                line: 1,
                heading: "Task 1".to_string(),
                content: String::new(),
                task_type: Some(TaskType::Todo),
                priority: Some(Priority::A),
                created: None,
                timestamp: None,
                timestamp_type: None,
                timestamp_date: None,
                timestamp_time: None,
                timestamp_end_time: None,
            },
            Task {
                file: "test.md".to_string(),
                line: 2,
                heading: "Task 2".to_string(),
                content: String::new(),
                task_type: Some(TaskType::Done),
                priority: Some(Priority::B),
                created: None,
                timestamp: None,
                timestamp_type: None,
                timestamp_date: None,
                timestamp_time: None,
                timestamp_end_time: None,
            },
        ];

        let result = filter_agenda(tasks, "tasks", None, None, None, "UTC").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].heading, "Task 1");
    }

    #[test]
    fn test_invalid_timezone() {
        let tasks = vec![];
        let result = filter_agenda(tasks, "day", None, None, None, "Invalid/Timezone");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid timezone"));
    }

    #[test]
    fn test_invalid_date_format() {
        let tasks = vec![];
        let result = filter_agenda(tasks, "day", Some("2024-13-01"), None, None, "UTC");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_mode() {
        let tasks = vec![];
        let result = filter_agenda(tasks, "invalid", None, None, None, "UTC");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid agenda mode"));
    }
}
