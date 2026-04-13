use rc_log_domain::asset::photo_service::PhotoServiceError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum RemoveUserPhotoError {
    #[error("User not found")]
    NotFound,
    #[error("Invalid user data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo service error: {0}")]
    PhotoServiceError(String),
}

impl From<TransactionError> for RemoveUserPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => RemoveUserPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => RemoveUserPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoServiceError> for RemoveUserPhotoError {
    fn from(err: PhotoServiceError) -> Self {
        match err {
            PhotoServiceError::InvalidData(msg) => RemoveUserPhotoError::PhotoServiceError(msg),
            PhotoServiceError::IoError(msg) => RemoveUserPhotoError::PhotoServiceError(msg),
            PhotoServiceError::DatabaseError(msg) => RemoveUserPhotoError::PhotoServiceError(msg),
        }
    }
}
