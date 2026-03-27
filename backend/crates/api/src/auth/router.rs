use axum::{Router, routing::post};

use crate::auth::sign_in::sign_in;
use crate::auth::sign_up::sign_up;
use crate::state::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/sign-up", post(sign_up))
        .route("/api/auth/sign-in", post(sign_in))
}
