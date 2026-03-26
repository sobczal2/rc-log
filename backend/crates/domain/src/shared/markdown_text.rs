use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkdownText(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkdownTextError {
    Empty,
}

impl fmt::Display for MarkdownTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownTextError::Empty => write!(f, "markdown text must not be empty"),
        }
    }
}

impl std::error::Error for MarkdownTextError {}

impl MarkdownText {
    pub fn new(value: String) -> Result<Self, MarkdownTextError> {
        if value.trim().is_empty() {
            return Err(MarkdownTextError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
