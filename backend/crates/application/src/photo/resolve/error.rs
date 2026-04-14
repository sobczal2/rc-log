use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum ResolvePhotoError {
    #[error("Photo asset not found")]
    NotFound,
    #[error("Invalid asset id: {0}")]
    InvalidId(String),
    #[error("Invalid data in repository: {0}")]
    InvalidData(String),
    #[error("Resolver error: {0}")]
    ResolverError(String),
}

impl From<TransactionError> for ResolvePhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => ResolvePhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => ResolvePhotoError::ResolverError(msg),
        }
    }
}
