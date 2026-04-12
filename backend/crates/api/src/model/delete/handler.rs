use axum::{extract::State, http::StatusCode};
use rc_log_application::model::delete::DeleteModelUseCase;
use rc_log_application::model::delete::model::DeleteModelInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::get_by_id::extractor::GetByIdRequest;
use crate::state::AppState;

#[instrument(skip(state), fields(model_id = %id.0, owner_id = %auth.id))]
pub async fn delete_model(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    id: GetByIdRequest,
) -> Result<StatusCode, ApiError> {
    debug!("Handling delete_model request");
    let mut use_case = DeleteModelUseCase::new(state.model_uow);
    use_case.execute(DeleteModelInput { id: id.0, owner_id: auth.id }).await?;
    debug!("Model deleted");
    Ok(StatusCode::NO_CONTENT)
}
