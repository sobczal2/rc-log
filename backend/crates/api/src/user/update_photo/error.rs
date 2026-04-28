use axum::response::{IntoResponse, Response};
use rc_log_application::user::update_photo::error::UpdateUserPhotoError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    NotFound,
    #[error("{0}")]
    InvalidPhotoContent(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdateUserPhotoError> for Error {
    fn from(e: UpdateUserPhotoError) -> Self {
        match e {
            UpdateUserPhotoError::NotFound => Self::NotFound,
            UpdateUserPhotoError::InvalidPhotoContent(msg) => Self::InvalidPhotoContent(msg),
            UpdateUserPhotoError::InvalidData(_) => Self::Internal,
            UpdateUserPhotoError::RepositoryError(_) => Self::Internal,
            UpdateUserPhotoError::PhotoServiceError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::InvalidPhotoContent(_) => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
