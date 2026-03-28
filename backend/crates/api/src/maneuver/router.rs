use axum::{Router, routing::get};

use crate::maneuver::get_by_id::get_maneuver_by_id;
use crate::maneuver::list::list_maneuvers;
use crate::state::AppState;

pub fn maneuver_router() -> Router<AppState> {
    Router::new()
        .route("/api/maneuvers", get(list_maneuvers))
        .route("/api/maneuvers/{id}", get(get_maneuver_by_id))
}
