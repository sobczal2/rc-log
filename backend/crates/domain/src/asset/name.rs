use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameError {
    Empty,
    TooLong,
}

impl fmt::Display for NameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NameError::Empty => write!(f, "asset name must not be empty"),
            NameError::TooLong => write!(f, "asset name must not exceed 255 characters"),
        }
    }
}

impl std::error::Error for NameError {}

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(NameError::Empty);
        }
        if trimmed.len() > 255 {
            return Err(NameError::TooLong);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Name, NameError};

    #[test]
    fn valid_name() {
        let name = Name::new("hero-clip".to_string()).unwrap();
        assert_eq!(name.as_str(), "hero-clip");
    }

    #[test]
    fn trims_whitespace() {
        let name = Name::new("  hero  ".to_string()).unwrap();
        assert_eq!(name.as_str(), "hero");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(Name::new("".to_string()), Err(NameError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(Name::new("   ".to_string()), Err(NameError::Empty));
    }

    #[test]
    fn exactly_255_chars_is_ok() {
        let s = "a".repeat(255);
        assert!(Name::new(s).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let s = "a".repeat(256);
        assert_eq!(Name::new(s), Err(NameError::TooLong));
    }
}
