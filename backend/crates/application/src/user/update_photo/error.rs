use rc_log_domain::asset::photo_service::PhotoServiceError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateUserPhotoError {
    #[error("User not found")]
    NotFound,
    #[error("Invalid photo content: {0}")]
    InvalidPhotoContent(String),
    #[error("Invalid user data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo service error: {0}")]
    PhotoServiceError(String),
}

impl From<TransactionError> for UpdateUserPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateUserPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateUserPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoServiceError> for UpdateUserPhotoError {
    fn from(err: PhotoServiceError) -> Self {
        match err {
            PhotoServiceError::InvalidData(msg) => UpdateUserPhotoError::InvalidPhotoContent(msg),
            PhotoServiceError::IoError(msg) => UpdateUserPhotoError::PhotoServiceError(msg),
            PhotoServiceError::DatabaseError(msg) => UpdateUserPhotoError::PhotoServiceError(msg),
        }
    }
}
