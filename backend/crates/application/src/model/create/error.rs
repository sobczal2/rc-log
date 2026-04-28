use rc_log_domain::{model::name::NameError, shared::transaction::TransactionError};

use crate::shared::validator::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum CreateModelError {
    #[error("Validation error: {0}")]
    Validation(ValidationError),
    #[error("Invalid model data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for CreateModelError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => CreateModelError::InvalidData(msg),
            TransactionError::TransactionError(msg) => CreateModelError::RepositoryError(msg),
        }
    }
}

impl From<NameError> for CreateModelError {
    fn from(err: NameError) -> Self {
        CreateModelError::Validation(ValidationError::new("name", &err.to_string()))
    }
}