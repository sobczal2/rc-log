use axum::{Router, routing::post};

use crate::session::create::create_session;
use crate::state::AppState;

pub fn session_router() -> Router<AppState> {
    Router::new().route("/api/sessions", post(create_session))
}
