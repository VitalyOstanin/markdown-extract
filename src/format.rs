use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Json,
    Markdown,
    Html,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "md" | "markdown" => Ok(OutputFormat::Markdown),
            "html" => Ok(OutputFormat::Html),
            _ => Err(format!("Invalid format: {s}. Valid formats: json, md, html")),
        }
    }
}
