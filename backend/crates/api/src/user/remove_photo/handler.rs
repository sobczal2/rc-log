use axum::extract::State;
use axum::http::StatusCode;
use rc_log_application::user::remove_photo::RemoveUserPhotoUseCase;
use rc_log_application::user::remove_photo::model::RemoveUserPhotoInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::state::AppState;

#[instrument(skip(state), fields(user_id = %auth.id))]
pub async fn remove_user_photo(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
) -> Result<StatusCode, ApiError> {
    debug!("Handling remove_user_photo request");
    let mut use_case = RemoveUserPhotoUseCase::new(state.user_uow, state.photo_service);
    use_case.execute(RemoveUserPhotoInput { user_id: auth.id }).await?;
    debug!("User photo removed");
    Ok(StatusCode::NO_CONTENT)
}
