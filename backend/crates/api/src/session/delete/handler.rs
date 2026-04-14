use axum::{extract::State, http::StatusCode};
use rc_log_application::session::delete::DeleteSessionUseCase;
use rc_log_application::session::delete::model::DeleteSessionInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::delete::extractor::DeleteRequest;
use crate::state::AppState;

#[instrument(skip(state), fields(session_id = %id.0, owner_id = %auth.id))]
pub async fn delete_session(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    id: DeleteRequest,
) -> Result<StatusCode, ApiError> {
    debug!("Handling delete_session request");

    let mut use_case = DeleteSessionUseCase::new(state.session_uow);
    use_case.execute(DeleteSessionInput { id: id.0, owner_id: auth.id }).await?;

    debug!("Session deleted");
    Ok(StatusCode::NO_CONTENT)
}
