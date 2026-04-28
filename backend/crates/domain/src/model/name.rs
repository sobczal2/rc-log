use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NameError {
    #[error("model name must not be empty")]
    Empty,
    #[error("model name must not exceed 255 characters")]
    TooLong,
}

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        if value.trim().is_empty() {
            return Err(NameError::Empty);
        }
        if value.len() > 255 {
            return Err(NameError::TooLong);
        }
        Ok(Self(value))
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
        let n = Name::new("My Trex 700".to_string()).unwrap();
        assert_eq!(n.as_str(), "My Trex 700");
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
        let name = "a".repeat(255);
        assert!(Name::new(name).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let name = "a".repeat(256);
        assert_eq!(Name::new(name), Err(NameError::TooLong));
    }
}
