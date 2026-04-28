use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::session::delete::error::DeleteSessionError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session not found")]
    NotFound,
    #[error("Access denied")]
    Forbidden,
    #[error("Internal server error")]
    Internal,
}

impl From<DeleteSessionError> for Error {
    fn from(e: DeleteSessionError) -> Self {
        match e {
            DeleteSessionError::NotFound => Self::NotFound,
            DeleteSessionError::Forbidden => Self::Forbidden,
            DeleteSessionError::RepositoryError(_) => Self::Internal,
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
