use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum ListModelsError {
    #[error("Invalid model data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for ListModelsError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => ListModelsError::InvalidData(msg),
            TransactionError::TransactionError(msg) => ListModelsError::RepositoryError(msg),
        }
    }
}
