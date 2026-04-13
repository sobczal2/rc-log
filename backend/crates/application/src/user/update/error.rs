use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserError {
    #[error("User not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Username already taken")]
    UsernameTaken,
    #[error("Invalid user data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for UpdateUserError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => {
                if msg == "unique_username" {
                    UpdateUserError::UsernameTaken
                } else {
                    UpdateUserError::InvalidData(msg)
                }
            }
            TransactionError::TransactionError(msg) => UpdateUserError::RepositoryError(msg),
        }
    }
}
