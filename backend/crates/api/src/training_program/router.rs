use axum::{Router, routing::get};

use crate::state::AppState;
use crate::training_program::get_by_id::get_training_program_by_id;

pub fn training_program_router() -> Router<AppState> {
    Router::new()
        .route("/api/training-programs/{id}", get(get_training_program_by_id))
}
