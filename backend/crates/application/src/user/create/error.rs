use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Username already exists")]
    UsernameTaken,
    #[error("Email already exists")]
    EmailTaken,
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for CreateUserError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => {
                if msg.contains("username") {
                    CreateUserError::UsernameTaken
                } else if msg.contains("email") {
                    CreateUserError::EmailTaken
                } else {
                    CreateUserError::RepositoryError(msg)
                }
            }
            TransactionError::TransactionError(msg) => CreateUserError::RepositoryError(msg),
        }
    }
}
