use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::model::update::error::UpdateModelError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdateModelError> for Error {
    fn from(e: UpdateModelError) -> Self {
        match e {
            UpdateModelError::NotFound => Self::NotFound,
            UpdateModelError::Forbidden => Self::Forbidden,
            UpdateModelError::ValidationError(msg) => Self::Validation(msg),
            UpdateModelError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in model update response");
                Self::Internal
            }
            UpdateModelError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::Forbidden => ApiError::Custom(StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
