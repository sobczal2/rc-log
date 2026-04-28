use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize)]
#[error("{field}: {message}")]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self { field: field.into(), message: message.into() }
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
