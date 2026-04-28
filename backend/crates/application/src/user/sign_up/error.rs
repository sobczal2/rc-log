use rc_log_domain::{shared::{email::{Email, EmailError}, password_hash::PasswordHashError, transaction::TransactionError}, user::username::UsernameError};

use crate::shared::validator::ValidationError;

#[derive(Debug, thiserror::Error)]
pub enum SignUpError {
    #[error("Validation error: {0:?}")]
    Validation(Vec<ValidationError>),
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

impl From<UsernameError> for SignUpError {
    fn from(err: UsernameError) -> Self {
        SignUpError::Validation(vec![ValidationError::new("username", &err.to_string())])
    }
}

impl From<EmailError> for SignUpError {
    fn from(err: EmailError) -> Self {
        SignUpError::Validation(vec![ValidationError::new("email", &err.to_string())])
    }
}

impl From<PasswordHashError> for SignUpError {
    fn from(err: PasswordHashError) -> Self {
        SignUpError::HashingError(err.to_string())
    }
}