use axum::{Json, extract::State};
use rc_log_application::training_program::get_by_id::GetTrainingProgramByIdUseCase;
use rc_log_application::training_program::get_by_id::model::GetTrainingProgramByIdInput;
use tracing::{debug, instrument};

use crate::state::AppState;
use crate::training_program::get_by_id::error::Error;
use crate::training_program::get_by_id::extractor::GetByIdRequest;
use crate::training_program::get_by_id::response::GetByIdResponse;

#[instrument(skip(state), fields(training_program_id = %id.0))]
pub async fn get_training_program_by_id(
    State(state): State<AppState>,
    id: GetByIdRequest,
) -> Result<Json<GetByIdResponse>, Error> {
    debug!("Handling get_training_program_by_id request");
    let mut use_case = GetTrainingProgramByIdUseCase::new(state.training_program_uow);
    let dto = use_case.execute(GetTrainingProgramByIdInput { id: id.0 }).await?;
    debug!("Training program found, returning response");
    Ok(Json(GetByIdResponse::from(dto)))
}
