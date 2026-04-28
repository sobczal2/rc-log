use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rc_log_application::model::get_by_id::error;
use serde_json::json;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Validation failed")]
    Validation(Vec<rc_log_application::shared::validator::ValidationError>),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    Internal,
    #[error("{0}")]
    Custom(StatusCode, String),
}

impl ApiError {
    pub fn bad_request(msg: impl ToString) -> Self {
        Self::Custom(StatusCode::BAD_REQUEST, msg.to_string())
    }

    pub fn not_found(msg: impl ToString) -> Self {
        Self::Custom(StatusCode::NOT_FOUND, msg.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!(error = %self, "request failed");
        match self {
            ApiError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response()
            }
            ApiError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Validation failed", "details": errors })),
            )
                .into_response(),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            ApiError::Custom(status, message) => (
                status,
                Json(json!({ "error": message })),
            ).into_response(),
        }
    }
}
