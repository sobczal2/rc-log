use axum::{Json, extract::State};
use rc_log_application::maneuver::list::ListManeuversUseCase;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::maneuver::list::extractor::ListRequest;
use crate::maneuver::list::response::ListResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(page = input.0.pagination.page))]
pub async fn list_maneuvers(
    State(state): State<AppState>,
    input: ListRequest,
) -> Result<Json<ListResponse>, ApiError> {
    debug!("Handling list maneuvers request");
    let mut use_case = ListManeuversUseCase::new(state.maneuver_uow);
    let result = use_case.execute(input.0).await?;
    debug!(total = result.total, count = result.items.len(), "Returning maneuver list");
    Ok(Json(ListResponse::from(result)))
}
