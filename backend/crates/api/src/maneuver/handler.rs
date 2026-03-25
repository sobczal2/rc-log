use axum::{
    Json,
    extract::{Path, State},
};
use rc_log_application::maneuver::get_by_id::GetManeuverByIdUseCase;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::error::ApiError;
use crate::maneuver::response::GetManeuverByIdResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(maneuver_id = %id))]
pub async fn get_maneuver_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetManeuverByIdResponse>, ApiError> {
    debug!("Handling get maneuver by id request");
    let mut use_case = GetManeuverByIdUseCase::new(state.maneuver_uow);
    let maneuver = use_case.execute(id).await?;
    debug!(name = maneuver.name(), "Maneuver found, returning response");
    Ok(Json(GetManeuverByIdResponse::from(maneuver)))
}
