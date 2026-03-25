#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MarkdownText(String);

impl MarkdownText {
    pub fn new(inner: String) -> Self {
        Self(inner)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
