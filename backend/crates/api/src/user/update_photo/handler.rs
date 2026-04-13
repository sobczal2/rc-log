use axum::Json;
use axum::extract::State;
use rc_log_application::user::update_photo::UpdateUserPhotoUseCase;
use rc_log_application::user::update_photo::model::UpdateUserPhotoInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::state::AppState;
use crate::user::update_photo::extractor::UpdateUserPhotoRequest;
use crate::user::update_photo::response::UpdateUserPhotoResponse;

#[instrument(skip(state, input), fields(user_id = %auth.id))]
pub async fn update_user_photo(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdateUserPhotoRequest,
) -> Result<Json<UpdateUserPhotoResponse>, ApiError> {
    debug!("Handling update_user_photo request");
    let mut use_case = UpdateUserPhotoUseCase::new(state.user_uow, state.photo_service);
    let dto = use_case
        .execute(UpdateUserPhotoInput { user_id: auth.id, data: input.data })
        .await?;
    debug!("User photo updated");
    Ok(Json(UpdateUserPhotoResponse::from(dto)))
}
