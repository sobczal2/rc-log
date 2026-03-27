use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum SignInError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid user data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for SignInError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => SignInError::InvalidData(msg),
            TransactionError::TransactionError(msg) => SignInError::RepositoryError(msg),
        }
    }
}
