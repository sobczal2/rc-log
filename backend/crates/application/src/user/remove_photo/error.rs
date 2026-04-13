use rc_log_domain::asset::photo_storage::PhotoStorageError;
use rc_log_domain::shared::transaction::TransactionError;

#[derive(Debug, thiserror::Error)]
pub enum RemoveUserPhotoError {
    #[error("User not found")]
    NotFound,
    #[error("Invalid user data: {0}")]
    InvalidData(String),
    #[error("Repository error: {0}")]
    RepositoryError(String),
    #[error("Photo storage error: {0}")]
    PhotoStorageError(String),
}

impl From<TransactionError> for RemoveUserPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => RemoveUserPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => RemoveUserPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoStorageError> for RemoveUserPhotoError {
    fn from(err: PhotoStorageError) -> Self {
        match err {
            PhotoStorageError::InvalidData(msg) => RemoveUserPhotoError::PhotoStorageError(msg),
            PhotoStorageError::IoError(msg) => RemoveUserPhotoError::PhotoStorageError(msg),
            PhotoStorageError::DatabaseError(msg) => RemoveUserPhotoError::PhotoStorageError(msg),
        }
    }
}
