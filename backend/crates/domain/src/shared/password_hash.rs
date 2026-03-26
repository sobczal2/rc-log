use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PasswordHash(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PasswordHashError {
    Empty,
}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordHashError::Empty => write!(f, "password hash must not be empty"),
        }
    }
}

impl std::error::Error for PasswordHashError {}

impl PasswordHash {
    pub fn new(value: String) -> Result<Self, PasswordHashError> {
        if value.is_empty() {
            return Err(PasswordHashError::Empty);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
