use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateModelError {
    #[error("Model not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Invalid model data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for UpdateModelError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateModelError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateModelError::RepositoryError(msg),
        }
    }
}
