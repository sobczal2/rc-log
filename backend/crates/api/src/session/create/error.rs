use axum::response::{IntoResponse, Response};
use rc_log_application::session::create::error::CreateSessionError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    ModelNotFound,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<CreateSessionError> for Error {
    fn from(e: CreateSessionError) -> Self {
        match e {
            CreateSessionError::ModelNotFound => Self::ModelNotFound,
            CreateSessionError::ValidationError(msg) => Self::Validation(msg),
            CreateSessionError::InvalidData(_) => Self::Internal,
            CreateSessionError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::ModelNotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
