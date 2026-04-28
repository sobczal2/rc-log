use axum::response::{IntoResponse, Response};
use rc_log_application::user::get_by_id::error::GetUserByIdError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<GetUserByIdError> for Error {
    fn from(e: GetUserByIdError) -> Self {
        match e {
            GetUserByIdError::NotFound => Self::NotFound,
            GetUserByIdError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in user get_by_id response");
                Self::Internal
            }
            GetUserByIdError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self.to_string()).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
