use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateSessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Model not found")]
    ModelNotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for UpdateSessionError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateSessionError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateSessionError::RepositoryError(msg),
        }
    }
}

impl From<ResolverError> for UpdateSessionError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::InvalidData(msg) => UpdateSessionError::InvalidData(msg),
            ResolverError::ResolverError(msg) => UpdateSessionError::RepositoryError(msg),
        }
    }
}
