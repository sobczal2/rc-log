use axum::response::{IntoResponse, Response};
use rc_log_application::user::sign_in::error::SignInError;
use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Internal server error")]
    Internal,
}

impl From<SignInError> for Error {
    fn from(e: SignInError) -> Self {
        match e {
            SignInError::InvalidCredentials => Self::InvalidCredentials,
            SignInError::InvalidData(_) => Self::Internal,
            SignInError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InvalidCredentials => ApiError::Unauthorized.into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
