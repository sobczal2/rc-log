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
use rc_log_application::model::remove_photo::error::RemoveModelPhotoError;
use rc_log_application::model::update::error::UpdateModelError;
use rc_log_application::model::update_photo::error::UpdateModelPhotoError;
use rc_log_application::photo::resolve::error::ResolvePhotoError;
use rc_log_application::session::add_performed_variation::error::AddPerformedVariationError;
use rc_log_application::session::create::error::CreateSessionError;
use rc_log_application::session::remove_performed_variation::error::RemovePerformedVariationError;
use rc_log_application::user::get_by_id::error::GetUserByIdError;
use rc_log_application::user::get_by_username::error::GetUserByUsernameError;
use rc_log_application::user::remove_photo::error::RemoveUserPhotoError;
use rc_log_application::user::sign_in::error::SignInError;
use rc_log_application::user::sign_up::error::SignUpError;
use rc_log_application::user::update::error::UpdateUserError;
use rc_log_application::user::update_photo::error::UpdateUserPhotoError;
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
    #[error("Internal server error")]
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!(error = %self, "request failed");
        match self {
            ApiError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, Json(json!({ "error": "Unauthorized" }))).into_response()
            }
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
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
                GetManeuverByIdError::InvalidData(ref msg),
            ))
            | ApiError::Application(ApplicationError::ListManeuvers(
                ListManeuversError::InvalidData(ref msg),
            )) => {
                error!(details = %msg, "invalid data in maneuver response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
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
                GetUserByIdError::InvalidData(ref msg),
            ))
            | ApiError::Application(ApplicationError::GetUserByUsername(
                GetUserByUsernameError::InvalidData(ref msg),
            )) => {
                error!(details = %msg, "invalid data in user response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
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
                GetModelByIdError::InvalidData(ref msg),
            ))
            | ApiError::Application(ApplicationError::ListModels(ListModelsError::InvalidData(
                ref msg,
            )))
            | ApiError::Application(ApplicationError::CreateModel(
                CreateModelError::InvalidData(ref msg),
            ))
            | ApiError::Application(ApplicationError::UpdateModel(
                UpdateModelError::InvalidData(ref msg),
            )) => {
                error!(details = %msg, "invalid data in model response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error" })),
                )
                    .into_response()
            }
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
            // Update model photo errors
            ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Model not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::Forbidden,
            )) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": "Access denied" }))).into_response()
            }
            ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::InvalidPhotoContent(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::UpdateModelPhoto(
                UpdateModelPhotoError::PhotoServiceError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Remove model photo errors
            ApiError::Application(ApplicationError::RemoveModelPhoto(
                RemoveModelPhotoError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Model not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::RemoveModelPhoto(
                RemoveModelPhotoError::Forbidden,
            )) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": "Access denied" }))).into_response()
            }
            ApiError::Application(ApplicationError::RemoveModelPhoto(
                RemoveModelPhotoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::RemoveModelPhoto(
                RemoveModelPhotoError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::RemoveModelPhoto(
                RemoveModelPhotoError::PhotoServiceError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Create session errors
            ApiError::Application(ApplicationError::CreateSession(
                CreateSessionError::ModelNotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Model not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::CreateSession(
                CreateSessionError::ValidationError(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::CreateSession(
                CreateSessionError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::CreateSession(
                CreateSessionError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Add performed variation errors
            ApiError::Application(ApplicationError::AddPerformedVariation(
                AddPerformedVariationError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Session not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::AddPerformedVariation(
                AddPerformedVariationError::Forbidden,
            )) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": "Access denied" }))).into_response()
            }
            ApiError::Application(ApplicationError::AddPerformedVariation(
                AddPerformedVariationError::ValidationError(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::AddPerformedVariation(
                AddPerformedVariationError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::AddPerformedVariation(
                AddPerformedVariationError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Remove performed variation errors
            ApiError::Application(ApplicationError::RemovePerformedVariation(
                RemovePerformedVariationError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Session not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::RemovePerformedVariation(
                RemovePerformedVariationError::PerformedVariationNotFound,
            )) => (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Performed variation not found" })),
            )
                .into_response(),
            ApiError::Application(ApplicationError::RemovePerformedVariation(
                RemovePerformedVariationError::Forbidden,
            )) => {
                (StatusCode::FORBIDDEN, Json(json!({ "error": "Access denied" }))).into_response()
            }
            ApiError::Application(ApplicationError::RemovePerformedVariation(
                RemovePerformedVariationError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::RemovePerformedVariation(
                RemovePerformedVariationError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Update user errors
            ApiError::Application(ApplicationError::UpdateUser(UpdateUserError::NotFound)) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::UpdateUser(
                UpdateUserError::UsernameTaken,
            )) => (StatusCode::CONFLICT, Json(json!({ "error": "Username already exists" })))
                .into_response(),
            ApiError::Application(ApplicationError::UpdateUser(
                UpdateUserError::ValidationError(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::UpdateUser(
                UpdateUserError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::UpdateUser(
                UpdateUserError::RepositoryError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Update user photo errors
            ApiError::Application(ApplicationError::UpdateUserPhoto(
                UpdateUserPhotoError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::UpdateUserPhoto(
                UpdateUserPhotoError::InvalidPhotoContent(msg),
            )) => (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response(),
            ApiError::Application(ApplicationError::UpdateUserPhoto(
                UpdateUserPhotoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::UpdateUserPhoto(
                UpdateUserPhotoError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::UpdateUserPhoto(
                UpdateUserPhotoError::PhotoServiceError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
            // Remove user photo errors
            ApiError::Application(ApplicationError::RemoveUserPhoto(
                RemoveUserPhotoError::NotFound,
            )) => {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "User not found" }))).into_response()
            }
            ApiError::Application(ApplicationError::RemoveUserPhoto(
                RemoveUserPhotoError::InvalidData(_),
            ))
            | ApiError::Application(ApplicationError::RemoveUserPhoto(
                RemoveUserPhotoError::RepositoryError(_),
            ))
            | ApiError::Application(ApplicationError::RemoveUserPhoto(
                RemoveUserPhotoError::PhotoServiceError(_),
            )) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Internal server error" })),
            )
                .into_response(),
        }
    }
}
