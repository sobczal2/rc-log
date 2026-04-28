use axum::{Json, extract::State};
use rc_log_application::video::resolve::ResolveVideoUseCase;
use tracing::{debug, instrument};

use crate::asset_paths::video::error::Error;
use crate::asset_paths::video::extractor::ResolveVideoRequest;
use crate::asset_paths::video::response::ResolveVideoResponse;
use crate::state::AppState;

#[instrument(skip(state), fields(video_id = %input.0.id))]
pub async fn resolve_video(
    State(state): State<AppState>,
    input: ResolveVideoRequest,
) -> Result<Json<ResolveVideoResponse>, Error> {
    debug!("Handling resolve video request");
    let use_case = ResolveVideoUseCase::new(state.video_resolver);
    let dto = use_case.execute(input.0).await?;
    debug!(video_id = %dto.id, "Video paths resolved");
    Ok(Json(ResolveVideoResponse::from(dto)))
}
