use axum::{Router, routing::get};

use crate::state::AppState;
use crate::user::get_by_id::get_user_by_id;

pub fn user_router() -> Router<AppState> {
    Router::new().route("/api/users/{id}", get(get_user_by_id))
}
