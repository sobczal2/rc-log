use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Resolver error: {0}")]
    ResolverError(String),
}
