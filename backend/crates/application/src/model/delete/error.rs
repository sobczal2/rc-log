use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum DeleteModelError {
    #[error("Model not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for DeleteModelError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => DeleteModelError::RepositoryError(msg),
            TransactionError::TransactionError(msg) => DeleteModelError::RepositoryError(msg),
        }
    }
}
