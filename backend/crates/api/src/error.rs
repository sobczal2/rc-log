use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use rc_log_application::error::ApplicationError;
use rc_log_application::maneuver::get_by_id::error::GetManeuverByIdError;
use rc_log_application::maneuver::list::error::ListManeuversError;
use rc_log_application::model::create::error::CreateModelError;
use rc_log_application::model::delete::error::DeleteModelError;
use rc_log_application::model::get_by_id::error::GetModelByIdError;
use rc_log_application::model::list::error::ListModelsError;
use rc_log_application::model::update::error::UpdateModelError;
use rc_log_application::photo::resolve::error::ResolvePhotoError;
use rc_log_application::user::get_by_id::error::GetUserByIdError;
use rc_log_application::user::get_by_username::error::GetUserByUsernameError;
use rc_log_application::user::sign_in::error::SignInError;
use rc_log_application::user::sign_up::error::SignUpError;
use rc_log_application::video::resolve::error::ResolveVideoError;
use serde_json::json;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
    #[error("Validation failed")]
    Validation(Vec<rc_log_application::shared::validator::ValidationError>),
    #[error("Unauthorized")]
    Unauthorized,
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
            ApiError::Application(ApplicationError::GetUserById(GetUserByIdError::NotFound)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::InvalidUsername,
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid username" })))
                .into_response(),
            ApiError::Application(ApplicationError::GetUserById(
                GetUserByIdError::InvalidData(msg),
            ))
            | ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::InvalidData(msg),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Internal server error: {}", msg) })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetUserById(
                GetUserByIdError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Sign-up errors
            ApiError::Application(ApplicationError::SignUp(SignUpError::UsernameTaken)) => {
                (StatusCode::CONFLICT, Json(json!({ "error": "Username already exists" })))
                    .into_response()
            }
            ApiError::Application(ApplicationError::SignUp(SignUpError::EmailTaken)) => {
                (StatusCode::CONFLICT, Json(json!({ "error": "Email already exists" })))
                    .into_response()
            }
            ApiError::Application(ApplicationError::SignUp(SignUpError::ValidationError(msg))) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response()
            }
            ApiError::Application(ApplicationError::SignUp(SignUpError::HashingError(_)))
            | ApiError::Application(ApplicationError::SignUp(SignUpError::RepositoryError(_))) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Sign-in errors
            ApiError::Application(ApplicationError::SignIn(SignInError::InvalidCredentials)) => {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Invalid credentials" })))
                    .into_response()
            }
            ApiError::Application(ApplicationError::SignIn(SignInError::InvalidData(_)))
            | ApiError::Application(ApplicationError::SignIn(SignInError::RepositoryError(_))) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Video asset paths errors
            ApiError::Application(ApplicationError::ResolveVideo(ResolveVideoError::NotFound)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Video asset not found" })))
                    .into_response()
            }
            ApiError::Application(ApplicationError::ResolveVideo(
                ResolveVideoError::InvalidName(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::ResolveVideo(
                ResolveVideoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::ResolveVideo(
                ResolveVideoError::ResolverError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Photo asset paths errors
            ApiError::Application(ApplicationError::ResolvePhoto(ResolvePhotoError::NotFound)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Photo asset not found" })))
                    .into_response()
            }
            ApiError::Application(ApplicationError::ResolvePhoto(
                ResolvePhotoError::InvalidName(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::ResolvePhoto(
                ResolvePhotoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::ResolvePhoto(
                ResolvePhotoError::ResolverError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Model errors
            ApiError::Application(ApplicationError::GetModelById(GetModelByIdError::NotFound))
            | ApiError::Application(ApplicationError::UpdateModel(UpdateModelError::NotFound))
            | ApiError::Application(ApplicationError::DeleteModel(DeleteModelError::NotFound)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Model not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::GetModelById(GetModelByIdError::Forbidden))
            | ApiError::Application(ApplicationError::UpdateModel(UpdateModelError::Forbidden))
            | ApiError::Application(ApplicationError::DeleteModel(DeleteModelError::Forbidden)) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": "Access denied" }))).into_response()
            }
            ApiError::Application(ApplicationError::CreateModel(
                CreateModelError::ValidationError(msg),
            ))
            | ApiError::Application(ApplicationError::UpdateModel(
                UpdateModelError::ValidationError(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::GetModelById(
                GetModelByIdError::InvalidData(msg),
            ))
            | ApiError::Application(ApplicationError::ListModels(
                ListModelsError::InvalidData(msg),
            ))
            | ApiError::Application(ApplicationError::CreateModel(
                CreateModelError::InvalidData(msg),
            ))
            | ApiError::Application(ApplicationError::UpdateModel(
                UpdateModelError::InvalidData(msg),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Internal server error: {}", msg) })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::GetModelById(
                GetModelByIdError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::ListModels(
                ListModelsError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::CreateModel(
                CreateModelError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::UpdateModel(
                UpdateModelError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::DeleteModel(
                DeleteModelError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
        }
    }
}
