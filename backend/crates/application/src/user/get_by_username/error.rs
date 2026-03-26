use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum GetUserByUsernameError {
    #[error("User not found")]
    NotFound,
    #[error("Invalid username")]
    InvalidUsername,
    #[error("Invalid user data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for GetUserByUsernameError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => GetUserByUsernameError::InvalidData(msg),
            TransactionError::TransactionError(msg) => GetUserByUsernameError::RepositoryError(msg),
        }
    }
}
