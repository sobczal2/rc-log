use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum AddPerformedVariationError {
    #[error("Session not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for AddPerformedVariationError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => AddPerformedVariationError::InvalidData(msg),
            TransactionError::TransactionError(msg) => {
                AddPerformedVariationError::RepositoryError(msg)
            }
        }
    }
}
