use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetNameError {
    Empty,
    TooLong,
}

impl fmt::Display for AssetNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetNameError::Empty => write!(f, "asset name must not be empty"),
            AssetNameError::TooLong => write!(f, "asset name must not exceed 255 characters"),
        }
    }
}

impl std::error::Error for AssetNameError {}

impl AssetName {
    pub fn new(value: String) -> Result<Self, AssetNameError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(AssetNameError::Empty);
        }
        if trimmed.len() > 255 {
            return Err(AssetNameError::TooLong);
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetName, AssetNameError};

    #[test]
    fn valid_name() {
        let name = AssetName::new("hero-clip".to_string()).unwrap();
        assert_eq!(name.as_str(), "hero-clip");
    }

    #[test]
    fn trims_whitespace() {
        let name = AssetName::new("  hero  ".to_string()).unwrap();
        assert_eq!(name.as_str(), "hero");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(AssetName::new("".to_string()), Err(AssetNameError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(AssetName::new("   ".to_string()), Err(AssetNameError::Empty));
    }

    #[test]
    fn exactly_255_chars_is_ok() {
        let s = "a".repeat(255);
        assert!(AssetName::new(s).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let s = "a".repeat(256);
        assert_eq!(AssetName::new(s), Err(AssetNameError::TooLong));
    }
}
