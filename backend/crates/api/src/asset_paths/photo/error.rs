use axum::{http::StatusCode, response::{IntoResponse, Response}};
use rc_log_application::photo::resolve::error::ResolvePhotoError;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Photo asset not found")]
    NotFound,
    #[error("Invalid asset id: {id}")]
    InvalidId{
        id: String,
    },
    #[error("Internal server error")]
    Internal,
}

impl From<ResolvePhotoError> for Error {
    fn from(e: ResolvePhotoError) -> Self {
        match e {
            ResolvePhotoError::NotFound => Self::NotFound,
            ResolvePhotoError::InvalidId{id} => Self::InvalidId{id},
            ResolvePhotoError::InvalidData(_) => Self::Internal,
            ResolvePhotoError::ResolverError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::InvalidId { .. } => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
