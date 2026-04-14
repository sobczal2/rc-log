use axum::{
    Router,
    routing::{get, post, put},
};

use crate::session::add_performed_variation::add_performed_variation;
use crate::session::create::create_session;
use crate::session::list::list_sessions;
use crate::session::remove_performed_variation::remove_performed_variation;
use crate::session::update::update_session;
use crate::session::update_performed_variation::update_performed_variation;
use crate::state::AppState;

pub fn session_router() -> Router<AppState> {
    Router::new()
        .route("/api/sessions", get(list_sessions).post(create_session))
        .route("/api/sessions/{id}", put(update_session))
        .route(
            "/api/sessions/{id}/performed-variations",
            post(add_performed_variation),
        )
        .route(
            "/api/sessions/{id}/performed-variations/{performed_variation_id}",
            put(update_performed_variation).delete(remove_performed_variation),
        )
}
