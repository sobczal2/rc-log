use rc_log_domain::asset::photo_storage::PhotoStorageError;
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
    #[error("Photo storage error: {0}")]
    PhotoStorageError(String),
}

impl From<TransactionError> for UpdateUserPhotoError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::InvalidData(msg) => UpdateUserPhotoError::InvalidData(msg),
            TransactionError::TransactionError(msg) => UpdateUserPhotoError::RepositoryError(msg),
        }
    }
}

impl From<PhotoStorageError> for UpdateUserPhotoError {
    fn from(err: PhotoStorageError) -> Self {
        match err {
            PhotoStorageError::InvalidData(msg) => UpdateUserPhotoError::InvalidPhotoContent(msg),
            PhotoStorageError::IoError(msg) => UpdateUserPhotoError::PhotoStorageError(msg),
            PhotoStorageError::DatabaseError(msg) => UpdateUserPhotoError::PhotoStorageError(msg),
        }
    }
}
