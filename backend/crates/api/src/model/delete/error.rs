use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::model::delete::error::DeleteModelError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Internal server error")]
    Internal,
}

impl From<DeleteModelError> for Error {
    fn from(e: DeleteModelError) -> Self {
        match e {
            DeleteModelError::NotFound => Self::NotFound,
            DeleteModelError::Forbidden => Self::Forbidden,
            DeleteModelError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::Forbidden => ApiError::Custom(StatusCode::FORBIDDEN, self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
