use axum::{Json, extract::State};
use rc_log_application::session::update::UpdateSessionUseCase;
use rc_log_application::session::update::model::UpdateSessionInput;
use tracing::{debug, instrument};

use crate::error::ApiError;
use crate::extractors::auth::AuthenticatedUser;
use crate::session::update::extractor::UpdateSessionRequest;
use crate::session::update::response::UpdateSessionResponse;
use crate::state::AppState;

#[instrument(skip(state, input), fields(session_id = %input.id, owner_id = %auth.id))]
pub async fn update_session(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    input: UpdateSessionRequest,
) -> Result<Json<UpdateSessionResponse>, ApiError> {
    debug!("Handling update_session request");

    let mut use_case = UpdateSessionUseCase::new(
        state.session_uow,
        state.model_resolver,
        state.maneuver_resolver,
        state.variation_resolver,
    );

    let session = use_case
        .execute(UpdateSessionInput {
            id: input.id,
            owner_id: auth.id,
            date: input.date,
            model_id: input.model_id,
            note: input.note,
        })
        .await?;

    Ok(Json(UpdateSessionResponse::from(session)))
}
