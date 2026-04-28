use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use rc_log_application::user::update::error::UpdateUserError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    NotFound,
    #[error("Username already exists")]
    UsernameTaken,
    #[error("{0}")]
    Validation(String),
    #[error("Internal server error")]
    Internal,
}

impl From<UpdateUserError> for Error {
    fn from(e: UpdateUserError) -> Self {
        match e {
            UpdateUserError::NotFound => Self::NotFound,
            UpdateUserError::UsernameTaken => Self::UsernameTaken,
            UpdateUserError::ValidationError(msg) => Self::Validation(msg),
            UpdateUserError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in user update response");
                Self::Internal
            }
            UpdateUserError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::UsernameTaken => ApiError::Custom(StatusCode::CONFLICT, self.to_string()).into_response(),
            Error::Validation(_) => ApiError::bad_request(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
