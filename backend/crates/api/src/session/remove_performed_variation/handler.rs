use axum::{Json, extract::State};
use rc_log_application::session::remove_performed_variation::RemovePerformedVariationUseCase;
use rc_log_application::session::remove_performed_variation::model::RemovePerformedVariationInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::remove_performed_variation::extractor::RemovePerformedVariationRequest;
use crate::session::remove_performed_variation::response::RemovePerformedVariationResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(session_id = %input.session_id, owner_id = %auth.id, variation_id = %input.variation_id))]
pub async fn remove_performed_variation(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: RemovePerformedVariationRequest,
) -> Result<Json<RemovePerformedVariationResponse>, ApiError> {
    debug!("Handling remove_performed_variation request");

    let mut use_case = RemovePerformedVariationUseCase::new(state.session_uow);
    let session = use_case
        .execute(RemovePerformedVariationInput {
            session_id: input.session_id,
            owner_id: auth.id,
            variation_id: input.variation_id,
        })
        .await?;

    Ok(Json(RemovePerformedVariationResponse::from(session)))
}
