use axum::response::{IntoResponse, Response};
use rc_log_application::model::update_photo::error::UpdateModelPhotoError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    NotFound,
    #[error("{0}")]
    InvalidPhotoContent(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdateModelPhotoError> for Error {
    fn from(e: UpdateModelPhotoError) -> Self {
        match e {
            UpdateModelPhotoError::NotFound => Self::NotFound,
            UpdateModelPhotoError::InvalidPhotoContent(msg) => Self::InvalidPhotoContent(msg),
            UpdateModelPhotoError::InvalidData(_) => Self::Internal,
            UpdateModelPhotoError::RepositoryError(_) => Self::Internal,
            UpdateModelPhotoError::PhotoServiceError(_) => Self::Internal,
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
