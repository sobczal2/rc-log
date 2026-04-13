use rc_log_domain::asset::photo_storage::PhotoStorageError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum UpdateModelPhotoError {
    #[error("Model not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Invalid photo content: {0}")]
    InvalidPhotoContent(String),
    #[error("Invalid model data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo storage error: {0}")]
    PhotoStorageError(String),
}

impl From<TransactionError> for UpdateModelPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateModelPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateModelPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoStorageError> for UpdateModelPhotoError {
    fn from(err: PhotoStorageError) -> Self {
        match err {
            PhotoStorageError::InvalidData(msg) => UpdateModelPhotoError::InvalidPhotoContent(msg),
            PhotoStorageError::IoError(msg) => UpdateModelPhotoError::PhotoStorageError(msg),
            PhotoStorageError::DatabaseError(msg) => UpdateModelPhotoError::PhotoStorageError(msg),
        }
    }
}
