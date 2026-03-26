use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rc_log_application::error::ApplicationError;
use rc_log_application::maneuver::get_by_id::error::GetManeuverByIdError;
use rc_log_application::maneuver::list::error::ListManeuversError;
use rc_log_application::user::create::error::CreateUserError;
use rc_log_application::user::get_by_id::error::GetUserByIdError;
use rc_log_application::user::get_by_username::error::GetUserByUsernameError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
    #[error("Validation failed")]
    Validation(Vec<rc_log_application::shared::validator::ValidationError>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Validation failed", "details": errors }))
            ).into_response(),
            ApiError::Application(ApplicationError::GetManeuverById(
                GetManeuverByIdError::NotFound,
            )) => (StatusCode::NOT_FOUND, Json(json!({ "error": "Maneuver not found" })))
                .into_response(),
            ApiError::Application(ApplicationError::GetManeuverById(
                GetManeuverByIdError::InvalidData(msg),
            ))
            | ApiError::Application(ApplicationError::ListManeuvers(
                ListManeuversError::InvalidData(msg),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Internal server error: {}", msg) })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetManeuverById(
                GetManeuverByIdError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::ListManeuvers(
                ListManeuversError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // User errors
            ApiError::Application(ApplicationError::GetUserById(GetUserByIdError::NotFound)) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "User not found" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::NotFound,
            )) => (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" })))
                .into_response(),
            ApiError::Application(ApplicationError::CreateUser(CreateUserError::UsernameTaken)) => (
                StatusCode::CONFLICT,
                Json(json!({ "error": "Username already exists" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::CreateUser(CreateUserError::EmailTaken)) => (
                StatusCode::CONFLICT,
                Json(json!({ "error": "Email already exists" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::CreateUser(
                CreateUserError::ValidationError(msg),
            )) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": msg })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::InvalidUsername,
            )) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid username" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetUserById(GetUserByIdError::InvalidData(msg)))
            | ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::InvalidData(msg),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Internal server error: {}", msg) })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetUserById(GetUserByIdError::RepositoryError(_)))
            | ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::CreateUser(CreateUserError::RepositoryError(_))) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
        }
    }
}
