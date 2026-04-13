use rc_log_domain::asset::photo_storage::PhotoStorageError;
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
    #[error("Photo storage error: {0}")]
    PhotoStorageError(String),
}

impl From<TransactionError> for RemoveModelPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => RemoveModelPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => RemoveModelPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoStorageError> for RemoveModelPhotoError {
    fn from(err: PhotoStorageError) -> Self {
        match err {
            PhotoStorageError::InvalidData(msg) => RemoveModelPhotoError::PhotoStorageError(msg),
            PhotoStorageError::IoError(msg) => RemoveModelPhotoError::PhotoStorageError(msg),
            PhotoStorageError::DatabaseError(msg) => RemoveModelPhotoError::PhotoStorageError(msg),
        }
    }
}
