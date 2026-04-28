use axum::response::{IntoResponse, Response};
use rc_log_application::session::remove_performed_variation::error::RemovePerformedVariationError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session not found")]
    NotFound,
    #[error("Performed variation not found")]
    PerformedVariationNotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<RemovePerformedVariationError> for Error {
    fn from(e: RemovePerformedVariationError) -> Self {
        match e {
            RemovePerformedVariationError::NotFound => Self::NotFound,
            RemovePerformedVariationError::PerformedVariationNotFound => {
                Self::PerformedVariationNotFound
            }
            RemovePerformedVariationError::InvalidData(_) => Self::Internal,
            RemovePerformedVariationError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::PerformedVariationNotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
