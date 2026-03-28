use axum::{Json, extract::State};
use rc_log_application::maneuver::get_by_id::GetManeuverByIdUseCase;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::maneuver::get_by_id::extractor::GetByIdRequest;
use crate::maneuver::get_by_id::response::GetByIdResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(maneuver_id = %input.0.id))]
pub async fn get_maneuver_by_id(
    State(state): State<AppState>,
    input: GetByIdRequest,
) -> Result<Json<GetByIdResponse>, ApiError> {
    debug!("Handling get maneuver by id request");
    let mut use_case = GetManeuverByIdUseCase::new(state.maneuver_uow);
    let maneuver = use_case.execute(input.0).await?;
    debug!(name = maneuver.name.as_str(), "Maneuver found, returning response");
    Ok(Json(GetByIdResponse::from(maneuver)))
}
