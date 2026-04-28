use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkdownText(String);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MarkdownTextError {
    #[error("markdown text must not be empty")]
    Empty,
}

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

#[cfg(test)]
mod tests {
    use super::{MarkdownText, MarkdownTextError};

    #[test]
    fn valid_text() {
        let t = MarkdownText::new("# Hello\nWorld".to_string()).unwrap();
        assert_eq!(t.as_str(), "# Hello\nWorld");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(MarkdownText::new("".to_string()), Err(MarkdownTextError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(MarkdownText::new("   ".to_string()), Err(MarkdownTextError::Empty));
    }

    #[test]
    fn preserves_markdown_syntax() {
        let md = "# Heading\n\n**bold** and *italic*";
        let t = MarkdownText::new(md.to_string()).unwrap();
        assert_eq!(t.as_str(), md);
    }
}
