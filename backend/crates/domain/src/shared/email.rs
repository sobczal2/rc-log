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
