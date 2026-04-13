use axum::Json;
use axum::extract::State;
use rc_log_application::model::update_photo::UpdateModelPhotoUseCase;
use rc_log_application::model::update_photo::model::UpdateModelPhotoInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::update_photo::extractor::UpdatePhotoRequest;
use crate::model::update_photo::response::UpdatePhotoResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(model_id = %input.model_id, owner_id = %auth.id))]
pub async fn update_model_photo(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdatePhotoRequest,
) -> Result<Json<UpdatePhotoResponse>, ApiError> {
    debug!("Handling update_model_photo request");
    let mut use_case = UpdateModelPhotoUseCase::new(state.model_uow, state.photo_service);
    let dto = use_case
        .execute(UpdateModelPhotoInput {
            model_id: input.model_id,
            owner_id: auth.id,
            data: input.data,
        })
        .await?;
    debug!("Model photo updated");
    Ok(Json(UpdatePhotoResponse::from(dto)))
}
