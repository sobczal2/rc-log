use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum GetManeuverByIdError {
    #[error("Maneuver not found")]
    NotFound,
    #[error("Invalid maneuver data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for GetManeuverByIdError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => GetManeuverByIdError::InvalidData(msg),
            TransactionError::TransactionError(msg) => GetManeuverByIdError::RepositoryError(msg),
        }
    }
}
