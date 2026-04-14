use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum ResolveVideoError {
    #[error("Video asset not found")]
    NotFound,
    #[error("Invalid asset id: {0}")]
    InvalidId(String),
    #[error("Invalid data in repository: {0}")]
    InvalidData(String),
    #[error("Resolver error: {0}")]
    ResolverError(String),
}

impl From<TransactionError> for ResolveVideoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => ResolveVideoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => ResolveVideoError::ResolverError(msg),
        }
    }
}
