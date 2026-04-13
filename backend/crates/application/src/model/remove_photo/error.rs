use rc_log_domain::asset::photo_service::PhotoServiceError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum RemoveModelPhotoError {
    #[error("Model not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Invalid model data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo service error: {0}")]
    PhotoServiceError(String),
}

impl From<TransactionError> for RemoveModelPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => RemoveModelPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => RemoveModelPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoServiceError> for RemoveModelPhotoError {
    fn from(err: PhotoServiceError) -> Self {
        match err {
            PhotoServiceError::InvalidData(msg) => RemoveModelPhotoError::PhotoServiceError(msg),
            PhotoServiceError::IoError(msg) => RemoveModelPhotoError::PhotoServiceError(msg),
            PhotoServiceError::DatabaseError(msg) => RemoveModelPhotoError::PhotoServiceError(msg),
        }
    }
}
