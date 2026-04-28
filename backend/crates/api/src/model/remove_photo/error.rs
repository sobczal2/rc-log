use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::model::remove_photo::error::RemoveModelPhotoError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<RemoveModelPhotoError> for Error {
    fn from(e: RemoveModelPhotoError) -> Self {
        match e {
            RemoveModelPhotoError::NotFound => Self::NotFound,
            RemoveModelPhotoError::InvalidData(_) => Self::Internal,
            RemoveModelPhotoError::RepositoryError(_) => Self::Internal,
            RemoveModelPhotoError::PhotoServiceError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
