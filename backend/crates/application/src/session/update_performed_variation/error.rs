use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdatePerformedVariationError {
    #[error("Session not found")]
    NotFound,
    #[error("Forbidden")]
    Forbidden,
    #[error("Performed variation not found")]
    PerformedVariationNotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for UpdatePerformedVariationError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdatePerformedVariationError::InvalidData(msg),
            TransactionError::TransactionError(msg) => {
                UpdatePerformedVariationError::RepositoryError(msg)
            }
        }
    }
}
