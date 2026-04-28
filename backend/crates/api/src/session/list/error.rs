use axum::response::{IntoResponse, Response};
use rc_log_application::session::list::error::ListSessionsError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal server error")]
    Internal,
}

impl From<ListSessionsError> for Error {
    fn from(e: ListSessionsError) -> Self {
        match e {
            ListSessionsError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in session list response");
                Self::Internal
            }
            ListSessionsError::RepositoryError(_) => Self::Internal,
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
