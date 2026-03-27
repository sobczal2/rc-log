use axum::{Json, extract::State};
use rc_log_application::user::get_by_id::GetUserByIdUseCase;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::state::AppState;
use crate::user::get_by_id::extractor::GetByIdRequest;
use crate::user::get_by_id::response::GetByIdResponse;

#[instrument(skip(state), fields(user_id = %input.0.id))]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    _auth: AuthenticatedUser,
    input: GetByIdRequest,
) -> Result<Json<GetByIdResponse>, ApiError> {
    debug!("Handling get_user_by_id request");
    let mut use_case = GetUserByIdUseCase::new(state.user_uow);
    let user = use_case.execute(input.0).await?;
    debug!(username = user.username.as_str(), "User found, returning response");
    Ok(Json(GetByIdResponse::from(user)))
}
