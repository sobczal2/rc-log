use axum::response::{IntoResponse, Response};
use rc_log_application::model::get_by_id::error::GetModelByIdError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<GetModelByIdError> for Error {
    fn from(e: GetModelByIdError) -> Self {
        match e {
            GetModelByIdError::NotFound => Self::NotFound,
            GetModelByIdError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in model get_by_id response");
                Self::Internal
            }
            GetModelByIdError::RepositoryError(_) => Self::Internal,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::NotFound => ApiError::not_found(self).into_response(),
            Error::Internal => ApiError::Internal.into_response(),
        }
    }
}
