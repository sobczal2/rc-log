use rc_log_domain::shared::repository::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum GetManeuverByIdError {
    #[error("Maneuver not found")]
    NotFound,
    #[error("Invalid maneuver data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<RepositoryError> for GetManeuverByIdError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::InvalidData(msg) => GetManeuverByIdError::InvalidData(msg),
            RepositoryError::TransactionError(msg) => GetManeuverByIdError::RepositoryError(msg),
        }
    }
}
