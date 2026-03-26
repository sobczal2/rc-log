use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsernameError {
    Empty,
    TooLong,
}

impl fmt::Display for UsernameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UsernameError::Empty => write!(f, "username must not be empty"),
            UsernameError::TooLong => write!(f, "username must not exceed 255 characters"),
        }
    }
}

impl std::error::Error for UsernameError {}

impl Username {
    pub fn new(value: String) -> Result<Self, UsernameError> {
        if value.trim().is_empty() {
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
