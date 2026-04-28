use axum::response::{IntoResponse, Response};
use rc_log_application::user::remove_photo::error::RemoveUserPhotoError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<RemoveUserPhotoError> for Error {
    fn from(e: RemoveUserPhotoError) -> Self {
        match e {
            RemoveUserPhotoError::NotFound => Self::NotFound,
            RemoveUserPhotoError::InvalidData(_) => Self::Internal,
            RemoveUserPhotoError::RepositoryError(_) => Self::Internal,
            RemoveUserPhotoError::PhotoServiceError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
