use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum GetTrainingProgramByIdError {
    #[error("Training program not found")]
    NotFound,
    #[error("Invalid training program data in repository: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

impl From<TransactionError> for GetTrainingProgramByIdError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => GetTrainingProgramByIdError::InvalidData(msg),
            TransactionError::TransactionError(msg) => {
                GetTrainingProgramByIdError::RepositoryError(msg)
            }
        }
    }
}
