use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

static TIMESTAMP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*((?:SCHEDULED|DEADLINE|CLOSED):\s*)<(\d{4}-\d{2}-\d{2}[^>]*)>")
        .expect("Invalid TIMESTAMP_RE regex")
});

static RANGE_TIMESTAMP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*<(\d{4}-\d{2}-\d{2}[^>]*)>--<(\d{4}-\d{2}-\d{2}[^>]*)>")
        .expect("Invalid RANGE_TIMESTAMP_RE regex")
});

static SIMPLE_TIMESTAMP_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*<(\d{4}-\d{2}-\d{2}[^>]*)>")
        .expect("Invalid SIMPLE_TIMESTAMP_RE regex")
});

static CREATED_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*CREATED:\s*<(\d{4}-\d{2}-\d{2}[^>]*)>")
        .expect("Invalid CREATED_RE regex")
});

/// Extract CREATED timestamp from text
pub fn extract_created(text: &str, mappings: &[(&str, &str)]) -> Option<String> {
    let text = normalize_weekdays(text, mappings);
    CREATED_RE.captures(&text).map(|caps| {
        format!("CREATED: <{}>", &caps[1])
    })
}

/// Extract non-CREATED timestamp from text
pub fn extract_timestamp(text: &str, mappings: &[(&str, &str)]) -> Option<String> {
    let text = normalize_weekdays(text, mappings);
    
    if let Some(caps) = TIMESTAMP_RE.captures(&text) {
        return Some(format!("{}<{}>", &caps[1], &caps[2]));
    }
    
    if let Some(caps) = RANGE_TIMESTAMP_RE.captures(&text) {
        return Some(format!("<{}>--<{}>", &caps[1], &caps[2]));
    }
    
    if let Some(caps) = SIMPLE_TIMESTAMP_RE.captures(&text) {
        return Some(format!("<{}>", &caps[1]));
    }
    
    None
}

/// Parse timestamp fields for JSON output
pub fn parse_timestamp_fields(
    timestamp: &str,
    mappings: &[(&str, &str)],
) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    let ts_type = if timestamp.contains("SCHEDULED:") {
        Some("SCHEDULED".to_string())
    } else if timestamp.contains("DEADLINE:") {
        Some("DEADLINE".to_string())
    } else if timestamp.contains("CLOSED:") {
        Some("CLOSED".to_string())
    } else {
        Some("PLAIN".to_string())
    };

    let normalized = normalize_weekdays(timestamp, mappings);
    
    static DATE_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(\d{4}-\d{2}-\d{2})").expect("Invalid DATE_RE")
    });
    static TIME_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(\d{1,2}:\d{2})(?:-(\d{1,2}:\d{2}))?").expect("Invalid TIME_RE")
    });

    let date = DATE_RE.captures(&normalized)
        .map(|caps| caps[1].to_string());
    
    let (time, end_time) = if let Some(caps) = TIME_RE.captures(&normalized) {
        (
            Some(caps[1].to_string()),
            caps.get(2).map(|m| m.as_str().to_string()),
        )
    } else {
        (None, None)
    };

    (ts_type, date, time, end_time)
}

fn normalize_weekdays<'a>(text: &'a str, mappings: &[(&str, &str)]) -> Cow<'a, str> {
    let mut result = Cow::Borrowed(text);
    for (localized, english) in mappings {
        if result.contains(localized) {
            result = Cow::Owned(result.replace(localized, english));
        }
    }
    result
}
