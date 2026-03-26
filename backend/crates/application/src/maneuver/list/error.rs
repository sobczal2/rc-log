use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum ListManeuversError {
    #[error("Invalid maneuver data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for ListManeuversError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => ListManeuversError::InvalidData(msg),
            TransactionError::TransactionError(msg) => ListManeuversError::RepositoryError(msg),
        }
    }
}
