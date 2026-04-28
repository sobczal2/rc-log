use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::session::update_performed_variation::error::UpdatePerformedVariationError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session not found")]
    NotFound,
    #[error("Performed variation not found")]
    PerformedVariationNotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdatePerformedVariationError> for Error {
    fn from(e: UpdatePerformedVariationError) -> Self {
        match e {
            UpdatePerformedVariationError::NotFound => Self::NotFound,
            UpdatePerformedVariationError::PerformedVariationNotFound => {
                Self::PerformedVariationNotFound
            }
            UpdatePerformedVariationError::Forbidden => Self::Forbidden,
            UpdatePerformedVariationError::ValidationError(msg) => Self::Validation(msg),
            UpdatePerformedVariationError::InvalidData(_) => Self::Internal,
            UpdatePerformedVariationError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::PerformedVariationNotFound => ApiError::not_found(self).into_response(),
            Error::Forbidden => ApiError::Custom(StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
