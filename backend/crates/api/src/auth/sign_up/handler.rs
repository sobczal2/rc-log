use axum::{Json, extract::State};
use rc_log_application::user::sign_up::SignUpUseCase;
use tracing::{debug, instrument};

use crate::auth::sign_up::extractor::SignUpRequest;
use crate::auth::sign_up::response::SignUpResponse;
use crate::error::ApiError;
use crate::jwt::{create_token, new_claims};
use crate::state::AppState;

#[instrument(skip(state, input))]
pub async fn sign_up(
    State(state): State<AppState>,
    input: SignUpRequest,
) -> Result<Json<SignUpResponse>, ApiError> {
    debug!("Handling sign_up request");
    let mut use_case = SignUpUseCase::new(state.user_uow);
    let user = use_case.execute(input.0).await?;
    let claims = new_claims(user.id, user.username.clone());
    let token = create_token(&claims, &state.jwt_secret).map_err(|_| ApiError::InternalError)?;
    Ok(Json(SignUpResponse { token, user }))
}
