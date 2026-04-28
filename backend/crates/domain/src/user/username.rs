use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum UsernameError {
    #[error("username must not be empty")]
    Empty,
    #[error("username must not exceed 255 characters")]
    TooLong,
}

impl Username {
    pub fn new(value: String) -> Result<Self, UsernameError> {
        let value = value.trim().to_string();
        if value.is_empty() {
            return Err(UsernameError::Empty);
        }
        if value.len() > 255 {
            return Err(UsernameError::TooLong);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Username, UsernameError};

    #[test]
    fn valid_username() {
        let u = Username::new("alice".to_string()).unwrap();
        assert_eq!(u.as_str(), "alice");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(Username::new("".to_string()), Err(UsernameError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(Username::new("   ".to_string()), Err(UsernameError::Empty));
    }

    #[test]
    fn exactly_255_chars_is_ok() {
        let s = "a".repeat(255);
        assert!(Username::new(s).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let s = "a".repeat(256);
        assert_eq!(Username::new(s), Err(UsernameError::TooLong));
    }
}
