use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum SignUpError {
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Username already exists")]
    UsernameTaken,
    #[error("Email already exists")]
    EmailTaken,
    #[error("Password hashing failed: {0}")]
    HashingError(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for SignUpError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) | TransactionError::TransactionError(msg) => {
                SignUpError::RepositoryError(msg)
            }
        }
    }
}
