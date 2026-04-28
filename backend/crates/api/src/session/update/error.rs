use axum::response::{IntoResponse, Response};
use rc_log_application::session::update::error::UpdateSessionError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Session not found")]
    NotFound,
    #[error("Model not found")]
    ModelNotFound,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdateSessionError> for Error {
    fn from(e: UpdateSessionError) -> Self {
        match e {
            UpdateSessionError::NotFound => Self::NotFound,
            UpdateSessionError::ModelNotFound => Self::ModelNotFound,
            UpdateSessionError::ValidationError(msg) => Self::Validation(msg),
            UpdateSessionError::InvalidData(_) => Self::Internal,
            UpdateSessionError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::ModelNotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
