use axum::{Json, extract::State};
use rc_log_application::session::list::ListSessionsUseCase;
use rc_log_application::session::list::model::ListSessionsInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::list::extractor::ListRequest;
use crate::session::list::response::ListResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(owner_id = %auth.id, page = input.pagination.page, page_size = input.pagination.page_size))]
pub async fn list_sessions(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: ListRequest,
) -> Result<Json<ListResponse>, ApiError> {
    debug!("Handling list_sessions request");

    let mut use_case = ListSessionsUseCase::new(state.session_uow);
    let result = use_case
        .execute(ListSessionsInput {
            owner_id: auth.id,
            pagination: input.pagination,
            filter: input.filter,
            sort: input.sort,
        })
        .await?;

    debug!(total = result.total, count = result.items.len(), "Returning session list");
    Ok(Json(ListResponse::from(result)))
}
