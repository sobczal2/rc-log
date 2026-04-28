use rc_log_domain::asset::photo_service::PhotoServiceError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateModelPhotoError {
    #[error("Model not found")]
    NotFound,
    #[error("Invalid photo content: {0}")]
    InvalidPhotoContent(String),
    #[error("Invalid model data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo service error: {0}")]
    PhotoServiceError(String),
}

impl From<TransactionError> for UpdateModelPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateModelPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateModelPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoServiceError> for UpdateModelPhotoError {
    fn from(err: PhotoServiceError) -> Self {
        match err {
            PhotoServiceError::InvalidData(msg) => UpdateModelPhotoError::InvalidPhotoContent(msg),
            PhotoServiceError::IoError(msg) => UpdateModelPhotoError::PhotoServiceError(msg),
            PhotoServiceError::DatabaseError(msg) => UpdateModelPhotoError::PhotoServiceError(msg),
        }
    }
}
