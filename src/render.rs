use crate::types::Task;

/// Render tasks as Markdown
pub fn render_markdown(tasks: &[Task]) -> String {
    let mut output = String::from("# Tasks\n\n");
    for task in tasks {
        output.push_str(&format!("## {}\n", task.heading));
        output.push_str(&format!("**File:** {}:{}\n", task.file, task.line));
        if let Some(ref t) = task.task_type {
            output.push_str(&format!("**Type:** {t:?}\n"));
        }
        if let Some(ref p) = task.priority {
            output.push_str(&format!("**Priority:** {p:?}\n"));
        }
        if let Some(ref c) = task.created {
            output.push_str(&format!("**Created:** {c}\n"));
        }
        if let Some(ref ts) = task.timestamp {
            output.push_str(&format!("**Time:** {ts}\n"));
        }
        if !task.content.is_empty() {
            output.push_str(&format!("\n{}\n\n", task.content));
        } else {
            output.push('\n');
        }
    }
    output
}

/// Render tasks as HTML
pub fn render_html(tasks: &[Task]) -> String {
    let mut output = String::from("<html><body><h1>Tasks</h1>\n");
    for task in tasks {
        output.push_str(&format!("<h2>{}</h2>\n", html_escape(&task.heading)));
        output.push_str(&format!(
            "<p><strong>File:</strong> {}:{}</p>\n",
            html_escape(&task.file),
            task.line
        ));
        if let Some(ref t) = task.task_type {
            output.push_str(&format!("<p><strong>Type:</strong> {t:?}</p>\n"));
        }
        if let Some(ref p) = task.priority {
            output.push_str(&format!("<p><strong>Priority:</strong> {p:?}</p>\n"));
        }
        if let Some(ref c) = task.created {
            output.push_str(&format!("<p><strong>Created:</strong> {}</p>\n", html_escape(c)));
        }
        if let Some(ref ts) = task.timestamp {
            output.push_str(&format!("<p><strong>Time:</strong> {}</p>\n", html_escape(ts)));
        }
        if !task.content.is_empty() {
            output.push_str(&format!("<p>{}</p>\n", html_escape(&task.content)));
        }
    }
    output.push_str("</body></html>");
    output
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Priority, TaskType};

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("A & B"), "A &amp; B");
    }

    #[test]
    fn test_render_markdown_basic() {
        let tasks = vec![Task {
            file: "test.md".to_string(),
            line: 1,
            heading: "Test Task".to_string(),
            content: "Description".to_string(),
            task_type: Some(TaskType::Todo),
            priority: Some(Priority::A),
            created: None,
            timestamp: None,
            timestamp_type: None,
            timestamp_date: None,
            timestamp_time: None,
            timestamp_end_time: None,
        }];

        let output = render_markdown(&tasks);
        assert!(output.contains("# Tasks"));
        assert!(output.contains("## Test Task"));
        assert!(output.contains("**Type:** Todo"));
    }

    #[test]
    fn test_render_html_escapes() {
        let tasks = vec![Task {
            file: "<script>.md".to_string(),
            line: 1,
            heading: "Test & Task".to_string(),
            content: String::new(),
            task_type: None,
            priority: None,
            created: None,
            timestamp: None,
            timestamp_type: None,
            timestamp_date: None,
            timestamp_time: None,
            timestamp_end_time: None,
        }];

        let output = render_html(&tasks);
        assert!(output.contains("&lt;script&gt;"));
        assert!(output.contains("Test &amp; Task"));
    }
}
