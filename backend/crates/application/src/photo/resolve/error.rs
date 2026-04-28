use rc_log_domain::shared::resolver::ResolverError;

#[derive(Debug, thiserror::Error)]
pub enum ResolvePhotoError {
    #[error("Photo asset not found")]
    NotFound,
    #[error("Invalid asset id: {id}")]
    InvalidId{
        id: String,
    },
    #[error("Invalid data in repository: {0}")]
    InvalidData(String),
    #[error("Resolver error: {0}")]
    ResolverError(String),
}

impl From<ResolverError> for ResolvePhotoError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::InvalidData(msg) => ResolvePhotoError::InvalidData(msg),
            ResolverError::ResolverError(msg) => ResolvePhotoError::ResolverError(msg),
        }
    }
}
