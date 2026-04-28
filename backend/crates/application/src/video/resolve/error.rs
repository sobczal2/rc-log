use rc_log_domain::shared::resolver::ResolverError;

#[derive(Debug, thiserror::Error)]
pub enum ResolveVideoError {
    #[error("Video asset not found")]
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

impl From<ResolverError> for ResolveVideoError {
    fn from(err: ResolverError) -> Self {
        match err {
            ResolverError::InvalidData(msg) => ResolveVideoError::InvalidData(msg),
            ResolverError::ResolverError(msg) => ResolveVideoError::ResolverError(msg),
        }
    }
}
