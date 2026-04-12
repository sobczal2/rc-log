use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelNameError {
    Empty,
    TooLong,
}

impl fmt::Display for ModelNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelNameError::Empty => write!(f, "model name must not be empty"),
            ModelNameError::TooLong => write!(f, "model name must not exceed 255 characters"),
        }
    }
}

impl std::error::Error for ModelNameError {}

impl ModelName {
    pub fn new(value: String) -> Result<Self, ModelNameError> {
        if value.trim().is_empty() {
            return Err(ModelNameError::Empty);
        }
        if value.len() > 255 {
            return Err(ModelNameError::TooLong);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{ModelName, ModelNameError};

    #[test]
    fn valid_name() {
        let n = ModelName::new("My Trex 700".to_string()).unwrap();
        assert_eq!(n.as_str(), "My Trex 700");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(ModelName::new("".to_string()), Err(ModelNameError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(ModelName::new("   ".to_string()), Err(ModelNameError::Empty));
    }

    #[test]
    fn exactly_255_chars_is_ok() {
        let name = "a".repeat(255);
        assert!(ModelName::new(name).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let name = "a".repeat(256);
        assert_eq!(ModelName::new(name), Err(ModelNameError::TooLong));
    }
}
