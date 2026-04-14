use axum::{Json, extract::State};
use rc_log_application::photo::resolve::ResolvePhotoUseCase;
use tracing::{debug, instrument};

use crate::asset_paths::photo::extractor::ResolvePhotoRequest;
use crate::asset_paths::photo::response::ResolvePhotoResponse;
use crate::error::ApiError;
use crate::state::AppState;

#[instrument(skip(state), fields(photo_id = %input.0.id))]
pub async fn resolve_photo(
    State(state): State<AppState>,
    input: ResolvePhotoRequest,
) -> Result<Json<ResolvePhotoResponse>, ApiError> {
    debug!("Handling resolve photo request");
    let use_case = ResolvePhotoUseCase::new(state.photo_resolver);
    let dto = use_case.execute(input.0).await?;
    debug!(photo_id = %dto.id, "Photo paths resolved");
    Ok(Json(ResolvePhotoResponse::from(dto)))
}
