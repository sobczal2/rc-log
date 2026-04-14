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
            NameError::Empty => write!(f, "training program name must not be empty"),
            NameError::TooLong => {
                write!(f, "training program name must not exceed 255 characters")
            }
        }
    }
}

impl std::error::Error for NameError {}

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
