use axum::{Json, extract::State, http::StatusCode};
use rc_log_application::session::create::CreateSessionUseCase;
use rc_log_application::session::create::model::CreateSessionInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::create::extractor::CreateSessionRequest;
use crate::session::create::response::CreateSessionResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(user_id = %auth.id, date = %input.date))]
pub async fn create_session(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: CreateSessionRequest,
) -> Result<(StatusCode, Json<CreateSessionResponse>), ApiError> {
    debug!("Handling create_session request");

    let mut use_case = CreateSessionUseCase::new(state.session_uow, state.model_uow);
    let session = use_case
        .execute(CreateSessionInput {
            user_id: auth.id,
            date: input.date,
            model_id: input.model_id,
            note: input.note,
        })
        .await?;

    debug!(session_id = %session.id, "Session created, returning response");
    Ok((StatusCode::CREATED, Json(CreateSessionResponse::from(session))))
}
