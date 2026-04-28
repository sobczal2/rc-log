use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NameError {
    #[error("training program name must not be empty")]
    Empty,
    #[error("training program name must not exceed 255 characters")]
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
