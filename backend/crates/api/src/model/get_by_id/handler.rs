use axum::{Json, extract::State};
use rc_log_application::model::get_by_id::GetModelByIdUseCase;
use rc_log_application::model::get_by_id::model::GetModelByIdInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::model::get_by_id::error::Error;
use crate::model::get_by_id::extractor::GetByIdRequest;
use crate::model::get_by_id::response::GetByIdResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(model_id = %id.0, owner_id = %auth.id))]
pub async fn get_model_by_id(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    id: GetByIdRequest,
) -> Result<Json<GetByIdResponse>, Error> {
    debug!("Handling get_model_by_id request");
    let mut use_case = GetModelByIdUseCase::new(state.model_uow);
    let dto = use_case.execute(GetModelByIdInput { id: id.0, owner_id: auth.id }).await?;
    debug!("Model found, returning response");
    Ok(Json(GetByIdResponse::from(dto)))
}
