use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum RemovePerformedVariationError {
    #[error("Session not found")]
    NotFound,
    #[error("Performed variation not found")]
    PerformedVariationNotFound,
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for RemovePerformedVariationError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => RemovePerformedVariationError::InvalidData(msg),
            TransactionError::TransactionError(msg) => {
                RemovePerformedVariationError::RepositoryError(msg)
            }
        }
    }
}
