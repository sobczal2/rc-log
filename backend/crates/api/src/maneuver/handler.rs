use axum::{
    extract::{Path, State},
    Json,
};
use rc_log_application::maneuver::get_by_id::GetManeuverByIdUseCase;
use uuid::Uuid;

use crate::error::ApiError;
use crate::maneuver::response::GetManeuverByIdResponse;
use crate::state::AppState;

pub async fn get_maneuver_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetManeuverByIdResponse>, ApiError> {
    let mut use_case = GetManeuverByIdUseCase::new(state.maneuver_uow);
    let maneuver = use_case.execute(id).await?;
    Ok(Json(GetManeuverByIdResponse::from(maneuver)))
}
