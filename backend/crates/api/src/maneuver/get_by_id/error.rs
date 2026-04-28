use axum::response::{IntoResponse, Response};
use rc_log_application::maneuver::get_by_id::error::GetManeuverByIdError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Maneuver not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<GetManeuverByIdError> for Error {
    fn from(e: GetManeuverByIdError) -> Self {
        match e {
            GetManeuverByIdError::NotFound => Self::NotFound,
            GetManeuverByIdError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in maneuver get_by_id response");
                Self::Internal
            }
            GetManeuverByIdError::RepositoryError(_) => Self::Internal,
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
