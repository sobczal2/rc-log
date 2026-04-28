use axum::response::{IntoResponse, Response};
use rc_log_application::model::create::error::CreateModelError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    ValidationError(String),
    #[error("Internal server error")]
    Internal,
}

impl From<CreateModelError> for Error {
    fn from(e: CreateModelError) -> Self {
        match e {
            CreateModelError::Validation(v) => Self::ValidationError(v.to_string()),
            CreateModelError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in model create response");
                Self::Internal
            }
            CreateModelError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ValidationError(_) => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
