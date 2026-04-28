use rc_log_domain::shared::resolver::ResolverError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum ListSessionsError {
    #[error("Invalid session data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for ListSessionsError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => ListSessionsError::InvalidData(msg),
            TransactionError::TransactionError(msg) => ListSessionsError::RepositoryError(msg),
        }
    }
}

impl From<ResolverError> for ListSessionsError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::InvalidData(msg) => ListSessionsError::InvalidData(msg),
            ResolverError::ResolverError(msg) => ListSessionsError::RepositoryError(msg),
        }
    }
}
