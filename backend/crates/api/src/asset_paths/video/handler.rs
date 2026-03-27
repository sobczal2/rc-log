use axum::{Json, extract::State};
use rc_log_application::video::resolve::ResolveVideoUseCase;
use tracing::{debug, instrument};

use crate::asset_paths::video::extractor::ResolveVideoRequest;
use crate::asset_paths::video::response::ResolveVideoResponse;
use crate::error::ApiError;
use crate::state::AppState;

#[instrument(skip(state), fields(name = %input.0.name))]
pub async fn resolve_video(
    State(state): State<AppState>,
    input: ResolveVideoRequest,
) -> Result<Json<ResolveVideoResponse>, ApiError> {
    debug!("Handling resolve video request");
    let use_case = ResolveVideoUseCase::new(state.video_resolver);
    let dto = use_case.execute(input.0).await?;
    debug!(name = %dto.name, "Video paths resolved");
    Ok(Json(ResolveVideoResponse::from(dto)))
}
