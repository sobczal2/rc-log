use axum::response::{IntoResponse, Response};
use rc_log_application::training_program::get_by_id::error::GetTrainingProgramByIdError;
use tracing::error;

use crate::error::ApiError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Training program not found")]
    NotFound,
    #[error("Internal server error")]
    Internal,
}

impl From<GetTrainingProgramByIdError> for Error {
    fn from(e: GetTrainingProgramByIdError) -> Self {
        match e {
            GetTrainingProgramByIdError::NotFound => Self::NotFound,
            GetTrainingProgramByIdError::InvalidData(msg) => {
                error!(details = %msg, "invalid data in training program get_by_id response");
                Self::Internal
            }
            GetTrainingProgramByIdError::RepositoryError(_) => Self::Internal,
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
