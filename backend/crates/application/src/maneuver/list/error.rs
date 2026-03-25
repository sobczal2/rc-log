use rc_log_domain::shared::repository::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum ListManeuversError {
    #[error("Invalid maneuver data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<RepositoryError> for ListManeuversError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::InvalidData(msg) => ListManeuversError::InvalidData(msg),
            RepositoryError::TransactionError(msg) => ListManeuversError::RepositoryError(msg),
        }
    }
}
