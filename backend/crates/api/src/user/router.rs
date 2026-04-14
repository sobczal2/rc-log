use axum::routing::{delete, put};
use axum::{Router, routing::get};

use crate::state::AppState;
use crate::user::get_by_id::get_user_by_id;
use crate::user::remove_photo::remove_user_photo;
use crate::user::update::update_user;
use crate::user::update_photo::update_user_photo;

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/api/users/{id}", get(get_user_by_id))
        .route("/api/users/me", put(update_user))
        .route("/api/users/me/photo", put(update_user_photo))
        .route("/api/users/me/photo", delete(remove_user_photo))
}
