use axum::response::{IntoResponse, Response};
use rc_log_application::model::list::error::ListModelsError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal server error")]
    Internal,
}

impl From<ListModelsError> for Error {
    fn from(e: ListModelsError) -> Self {
        match e {
            ListModelsError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in model list response");
                Self::Internal
            }
            ListModelsError::RepositoryError(_) => Self::Internal,
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
