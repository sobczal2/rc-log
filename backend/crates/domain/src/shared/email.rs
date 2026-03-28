use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    TooLong,
    InvalidFormat,
}

impl fmt::Display for EmailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailError::Empty => write!(f, "email must not be empty"),
            EmailError::TooLong => write!(f, "email must not exceed 255 characters"),
            EmailError::InvalidFormat => write!(f, "email must contain '@'"),
        }
    }
}

impl std::error::Error for EmailError {}

impl Email {
    pub fn new(value: String) -> Result<Self, EmailError> {
        if value.trim().is_empty() {
            return Err(EmailError::Empty);
        }
        if value.len() > 255 {
            return Err(EmailError::TooLong);
        }
        if !value.contains('@') {
            return Err(EmailError::InvalidFormat);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{Email, EmailError};

    #[test]
    fn valid_email() {
        let e = Email::new("user@example.com".to_string()).unwrap();
        assert_eq!(e.as_str(), "user@example.com");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(Email::new("".to_string()), Err(EmailError::Empty));
    }

    #[test]
    fn whitespace_only_is_err() {
        assert_eq!(Email::new("   ".to_string()), Err(EmailError::Empty));
    }

    #[test]
    fn missing_at_sign_is_err() {
        assert_eq!(Email::new("userexample.com".to_string()), Err(EmailError::InvalidFormat));
    }

    #[test]
    fn exactly_255_chars_is_ok() {
        // "a".repeat(251) + "@b.c" = 255 chars
        let email = format!("{}@b.c", "a".repeat(251));
        assert_eq!(email.len(), 255);
        assert!(Email::new(email).is_ok());
    }

    #[test]
    fn over_255_chars_is_err() {
        let email = format!("{}@b.c", "a".repeat(252));
        assert_eq!(Email::new(email), Err(EmailError::TooLong));
    }
}
