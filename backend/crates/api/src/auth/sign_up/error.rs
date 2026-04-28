use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::user::sign_up::error::SignUpError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Username already exists")]
    UsernameTaken,
    #[error("Email already exists")]
    EmailTaken,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<SignUpError> for Error {
    fn from(e: SignUpError) -> Self {
        match e {
            SignUpError::UsernameTaken => Self::UsernameTaken,
            SignUpError::EmailTaken => Self::EmailTaken,
            SignUpError::ValidationError(msg) => Self::Validation(msg),
            SignUpError::HashingError(_) => Self::Internal,
            SignUpError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::UsernameTaken => ApiError::Custom(StatusCode::CONFLICT, self.to_string()).into_response(),
            Error::EmailTaken => ApiError::Custom(StatusCode::CONFLICT, self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
