use axum::{Json, extract::State};
use rc_log_application::model::update::UpdateModelUseCase;
use rc_log_application::model::update::model::UpdateModelInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::model::update::extractor::UpdateRequest;
use crate::model::update::response::UpdateResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(model_id = %input.id, owner_id = %auth.id))]
pub async fn update_model(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdateRequest,
) -> Result<Json<UpdateResponse>, ApiError> {
    debug!("Handling update_model request");
    let mut use_case = UpdateModelUseCase::new(state.model_uow);
    let dto = use_case
        .execute(UpdateModelInput {
            id: input.id,
            owner_id: auth.id,
            name: input.name,
            r#type: input.r#type,
        })
        .await?;
    debug!("Model updated, returning response");
    Ok(Json(UpdateResponse::from(dto)))
}
