use axum::{
    Router,
    routing::{delete, post},
};

use crate::session::add_performed_variation::add_performed_variation;
use crate::session::create::create_session;
use crate::session::remove_performed_variation::remove_performed_variation;
use crate::state::AppState;

pub fn session_router() -> Router<AppState> {
    Router::new()
        .route("/api/sessions", post(create_session))
        .route(
            "/api/sessions/{id}/performed-variations",
            post(add_performed_variation),
        )
        .route(
            "/api/sessions/{id}/performed-variations/{variation_id}",
            delete(remove_performed_variation),
        )
}
