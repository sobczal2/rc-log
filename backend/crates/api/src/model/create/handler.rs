use axum::{Json, extract::State, http::StatusCode};
use rc_log_application::model::create::CreateModelUseCase;
use rc_log_application::model::create::model::CreateModelInput;
use tracing::{debug, instrument};

use crate::extractors::auth::AuthenticatedUser;
use crate::model::create::error::Error;
use crate::model::create::extractor::CreateRequest;
use crate::model::create::response::CreateResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(owner_id = %auth.id, name = %input.name))]
pub async fn create_model(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: CreateRequest,
) -> Result<(StatusCode, Json<CreateResponse>), Error> {
    debug!("Handling create_model request");
    let mut use_case = CreateModelUseCase::new(state.model_uow);
    let dto = use_case
        .execute(CreateModelInput { owner_id: auth.id, name: input.name, r#type: input.r#type })
        .await?;
    debug!(model_id = %dto.id, "Model created, returning response");
    Ok((StatusCode::CREATED, Json(CreateResponse::from(dto))))
}
