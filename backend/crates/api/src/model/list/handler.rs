use axum::{Json, extract::State};
use rc_log_application::model::list::ListModelsUseCase;
use rc_log_application::model::list::model::ListModelsInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::model::list::error::Error;
use crate::model::list::extractor::ListRequest;
use crate::model::list::response::ListResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(owner_id = %auth.id, page = %pagination.0.page, page_size = %pagination.0.page_size))]
pub async fn list_models(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    pagination: ListRequest,
) -> Result<Json<ListResponse>, Error> {
    debug!("Handling list_models request");
    let mut use_case = ListModelsUseCase::new(state.model_uow);
    let result =
        use_case.execute(ListModelsInput { owner_id: auth.id, pagination: pagination.0 }).await?;
    debug!(total = result.total, "Models retrieved, returning response");
    Ok(Json(ListResponse::from(result)))
}
