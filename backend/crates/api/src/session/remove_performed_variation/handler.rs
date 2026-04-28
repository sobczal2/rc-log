use axum::{extract::State, http::StatusCode};
use rc_log_application::session::remove_performed_variation::RemovePerformedVariationUseCase;
use rc_log_application::session::remove_performed_variation::model::RemovePerformedVariationInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::session::remove_performed_variation::error::Error;
use crate::session::remove_performed_variation::extractor::RemovePerformedVariationRequest;
use crate::state::AppState;

#[instrument(skip(state, input), fields(session_id = %input.session_id, owner_id = %auth.id, performed_variation_id = %input.performed_variation_id))]
pub async fn remove_performed_variation(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: RemovePerformedVariationRequest,
) -> Result<StatusCode, Error> {
    debug!("Handling remove_performed_variation request");
    let mut use_case = RemovePerformedVariationUseCase::new(state.session_uow);
    use_case
        .execute(RemovePerformedVariationInput {
            session_id: input.session_id,
            owner_id: auth.id,
            performed_variation_id: input.performed_variation_id,
        })
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
