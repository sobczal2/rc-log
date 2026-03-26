use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum GetUserByIdError {
    #[error("User not found")]
    NotFound,
    #[error("Invalid user data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for GetUserByIdError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => GetUserByIdError::InvalidData(msg),
            TransactionError::TransactionError(msg) => GetUserByIdError::RepositoryError(msg),
        }
    }
}
