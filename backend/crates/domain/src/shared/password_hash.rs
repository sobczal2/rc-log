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

#[cfg(test)]
mod tests {
    use super::{PasswordHash, PasswordHashError};

    #[test]
    fn valid_hash() {
        let h = PasswordHash::new("$argon2id$v=19$hash".to_string()).unwrap();
        assert_eq!(h.as_str(), "$argon2id$v=19$hash");
    }

    #[test]
    fn empty_is_err() {
        assert_eq!(PasswordHash::new("".to_string()), Err(PasswordHashError::Empty));
    }

    #[test]
    fn non_empty_whitespace_is_ok() {
        // PasswordHash only rejects truly empty strings; whitespace is unusual but valid
        assert!(PasswordHash::new(" ".to_string()).is_ok());
    }
}
