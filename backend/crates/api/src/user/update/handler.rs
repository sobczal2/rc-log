use axum::Json;
use axum::extract::State;
use rc_log_application::user::update::UpdateUserUseCase;
use rc_log_application::user::update::model::UpdateUserInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::state::AppState;
use crate::user::update::extractor::UpdateUserRequest;
use crate::user::update::response::UpdateUserResponse;

#[instrument(skip(state, input), fields(user_id = %auth.id, new_username = %input.new_username))]
pub async fn update_user(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdateUserRequest,
) -> Result<Json<UpdateUserResponse>, ApiError> {
    debug!("Handling update_user request");
    let mut use_case = UpdateUserUseCase::new(state.user_uow);
    let dto = use_case
        .execute(UpdateUserInput { user_id: auth.id, new_username: input.new_username })
        .await?;
    debug!("User updated");
    Ok(Json(UpdateUserResponse::from(dto)))
}
