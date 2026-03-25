use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use rc_log_application::error::ApplicationError;
use rc_log_application::maneuver::error::ManeuverError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Application(ApplicationError::Maneuver(ManeuverError::NotFound)) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Maneuver not found" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::Maneuver(
                ManeuverError::InvalidData(msg),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Internal server error: {}", msg) })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::Maneuver(
                ManeuverError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
        }
    }
}
