use axum::{Json, extract::State};
use rc_log_application::user::sign_in::SignInUseCase;
use tracing::{debug, instrument};

use crate::auth::sign_in::error::Error;
use crate::auth::sign_in::extractor::SignInRequest;
use crate::auth::sign_in::response::SignInResponse;
use crate::jwt::{create_token, new_claims};
use crate::state::AppState;

#[instrument(skip(state, input))]
pub async fn sign_in(
    State(state): State<AppState>,
    input: SignInRequest,
) -> Result<Json<SignInResponse>, Error> {
    debug!("Handling sign_in request");
    let mut use_case = SignInUseCase::new(state.user_uow);
    let user = use_case.execute(input.0).await?;
    let claims = new_claims(user.id, user.username.clone());
    let token = create_token(&claims, &state.jwt_secret).map_err(|_| Error::Internal)?;
    Ok(Json(SignInResponse { token, user }))
}
