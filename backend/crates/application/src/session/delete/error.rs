use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum DeleteSessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for DeleteSessionError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => DeleteSessionError::RepositoryError(msg),
            TransactionError::TransactionError(msg) => DeleteSessionError::RepositoryError(msg),
        }
    }
}
