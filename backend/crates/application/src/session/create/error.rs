use rc_log_domain::shared::transaction::TransactionError;
use rc_log_domain::shared::resolver::ResolverError;

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionError {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Model not found")]
    ModelNotFound,
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for CreateSessionError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => CreateSessionError::InvalidData(msg),
            TransactionError::TransactionError(msg) => CreateSessionError::RepositoryError(msg),
        }
    }
}

impl From<ResolverError> for CreateSessionError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::InvalidData(msg) => CreateSessionError::InvalidData(msg),
            ResolverError::ResolverError(msg) => CreateSessionError::RepositoryError(msg),
        }
    }
}
