use axum::response::{IntoResponse, Response};
use rc_log_application::maneuver::list::error::ListManeuversError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal server error")]
    Internal,
}

impl From<ListManeuversError> for Error {
    fn from(e: ListManeuversError) -> Self {
        match e {
            ListManeuversError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in maneuver list response");
                Self::Internal
            }
            ListManeuversError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
