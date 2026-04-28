use axum::{extract::State, http::StatusCode};
use rc_log_application::model::remove_photo::RemoveModelPhotoUseCase;
use rc_log_application::model::remove_photo::model::RemoveModelPhotoInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::model::remove_photo::error::Error;
use crate::model::remove_photo::extractor::RemovePhotoRequest;
use crate::state::AppState;

#[instrument(skip(state), fields(model_id = %id.0, owner_id = %auth.id))]
pub async fn remove_model_photo(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    id: RemovePhotoRequest,
) -> Result<StatusCode, Error> {
    debug!("Handling remove_model_photo request");
    let mut use_case = RemoveModelPhotoUseCase::new(state.model_uow, state.photo_service);
    use_case.execute(RemoveModelPhotoInput { model_id: id.0, owner_id: auth.id }).await?;
    debug!("Model photo removed");
    Ok(StatusCode::NO_CONTENT)
}
