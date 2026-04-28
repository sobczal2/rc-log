use axum::{response::{IntoResponse, Response}};
use rc_log_application::video::resolve::error::ResolveVideoError;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Video asset not found")]
    NotFound,
    #[error("Invalid video asset id: {id}")]
    InvalidId {
        id: String,
    },
    #[error("Internal server error")]
    Internal,
}

impl From<ResolveVideoError> for Error {
    fn from(e: ResolveVideoError) -> Self {
        match e {
            ResolveVideoError::NotFound => Self::NotFound,
            ResolveVideoError::InvalidId { id } => Self::InvalidId { id },
            ResolveVideoError::InvalidData(_) => Self::Internal,
            ResolveVideoError::ResolverError(_) => Self::Internal,
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
