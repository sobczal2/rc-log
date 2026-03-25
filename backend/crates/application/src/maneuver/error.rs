use rc_log_domain::shared::repository::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum ManeuverError {
    #[error("Maneuver not found")]
    NotFound,
    #[error("Invalid maneuver data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<RepositoryError> for ManeuverError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::InvalidData(msg) => ManeuverError::InvalidData(msg),
            RepositoryError::TransactionError(msg) => ManeuverError::RepositoryError(msg),
        }
    }
}
