use axum::{Json, extract::State};
use rc_log_application::session::add_performed_variation::AddPerformedVariationUseCase;
use rc_log_application::session::add_performed_variation::model::AddPerformedVariationInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::session::add_performed_variation::error::Error;
use crate::session::add_performed_variation::extractor::AddPerformedVariationRequest;
use crate::session::add_performed_variation::response::AddPerformedVariationResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(session_id = %input.session_id, owner_id = %auth.id, variation_id = %input.variation_id))]
pub async fn add_performed_variation(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: AddPerformedVariationRequest,
) -> Result<Json<AddPerformedVariationResponse>, Error> {
    debug!("Handling add_performed_variation request");
    let mut use_case = AddPerformedVariationUseCase::new(
        state.session_uow,
        state.model_resolver,
        state.maneuver_resolver,
        state.variation_resolver,
    );
    let session = use_case
        .execute(AddPerformedVariationInput {
            session_id: input.session_id,
            owner_id: auth.id,
            variation_id: input.variation_id,
            quality: input.quality,
            comfort: input.comfort,
            repeatability: input.repeatability,
            note: input.note,
        })
        .await?;
    Ok(Json(AddPerformedVariationResponse::from(session)))
}
