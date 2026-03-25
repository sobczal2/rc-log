use axum::{
    Json,
    extract::{Path, State},
};
use rc_log_application::maneuver::get_by_id::GetManeuverByIdUseCase;
use rc_log_application::maneuver::list::ListManeuversUseCase;
use tracing::{debug, instrument};
use uuid::Uuid;

use crate::error::ApiError;
use crate::maneuver::response::{GetManeuverByIdResponse, ListManeuversResponse};
use crate::shared::pagination::PaginationQuery;
use crate::state::AppState;

#[instrument(skip(state), fields(maneuver_id = %id))]
pub async fn get_maneuver_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetManeuverByIdResponse>, ApiError> {
    debug!("Handling get maneuver by id request");
    let mut use_case = GetManeuverByIdUseCase::new(state.maneuver_uow);
    let maneuver = use_case.execute(id).await?;
    debug!(name = maneuver.name.as_str(), "Maneuver found, returning response");
    Ok(Json(GetManeuverByIdResponse::from(maneuver)))
}

#[instrument(skip(state))]
pub async fn list_maneuvers(
    State(state): State<AppState>,
    pagination: PaginationQuery,
) -> Result<Json<ListManeuversResponse>, ApiError> {
    debug!(page = pagination.page, page_size = pagination.page_size, "Handling list maneuvers request");
    let domain_pagination = pagination.into_pagination();
    let mut use_case = ListManeuversUseCase::new(state.maneuver_uow);
    let result = use_case.execute(domain_pagination).await?;
    debug!(total = result.total, count = result.items.len(), "Returning maneuver list");
    Ok(Json(ListManeuversResponse::from(result)))
}
