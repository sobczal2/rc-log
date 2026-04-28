use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum GetModelByIdError {
    #[error("Model not found")]
    NotFound,
    #[error("Invalid model data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for GetModelByIdError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => GetModelByIdError::InvalidData(msg),
            TransactionError::TransactionError(msg) => GetModelByIdError::RepositoryError(msg),
        }
    }
}
