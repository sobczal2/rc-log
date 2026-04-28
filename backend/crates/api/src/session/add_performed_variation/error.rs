use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::session::add_performed_variation::error::AddPerformedVariationError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<AddPerformedVariationError> for Error {
    fn from(e: AddPerformedVariationError) -> Self {
        match e {
            AddPerformedVariationError::NotFound => Self::NotFound,
            AddPerformedVariationError::Forbidden => Self::Forbidden,
            AddPerformedVariationError::ValidationError(msg) => Self::Validation(msg),
            AddPerformedVariationError::InvalidData(_) => Self::Internal,
            AddPerformedVariationError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Forbidden => ApiError::Custom(StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
