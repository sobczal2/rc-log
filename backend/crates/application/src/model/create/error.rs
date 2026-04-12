use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum CreateModelError {
    #[error("Validation error: {0}")]
    ValidationError(String),
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
