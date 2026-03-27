use axum::{Router, routing::get};

use crate::asset_paths::photo::resolve_photo;
use crate::asset_paths::video::resolve_video;
use crate::state::AppState;

pub fn asset_paths_router() -> Router<AppState> {
    Router::new()
        .route("/api/asset-paths/video/{name}", get(resolve_video))
        .route("/api/asset-paths/photo/{name}", get(resolve_photo))
}
