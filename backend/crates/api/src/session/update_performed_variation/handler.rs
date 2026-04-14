use axum::{extract::State, http::StatusCode};
use rc_log_application::session::update_performed_variation::UpdatePerformedVariationUseCase;
use rc_log_application::session::update_performed_variation::model::UpdatePerformedVariationInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::update_performed_variation::extractor::UpdatePerformedVariationRequest;
use crate::state::AppState;

#[instrument(skip(state, input), fields(session_id = %input.session_id, owner_id = %auth.id, performed_variation_id = %input.performed_variation_id))]
pub async fn update_performed_variation(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdatePerformedVariationRequest,
) -> Result<StatusCode, ApiError> {
    debug!("Handling update_performed_variation request");

    let mut use_case = UpdatePerformedVariationUseCase::new(state.session_uow);
    use_case
        .execute(UpdatePerformedVariationInput {
            session_id: input.session_id,
            owner_id: auth.id,
            performed_variation_id: input.performed_variation_id,
            quality: input.quality,
            comfort: input.comfort,
            repeatability: input.repeatability,
            note: input.note,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
