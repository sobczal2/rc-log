use axum::{Router, routing::get};

use crate::model::create::create_model;
use crate::model::delete::delete_model;
use crate::model::get_by_id::get_model_by_id;
use crate::model::list::list_models;
use crate::model::update::update_model;
use crate::state::AppState;

pub fn model_router() -> Router<AppState> {
    Router::new()
        .route("/api/models", get(list_models).post(create_model))
        .route(
            "/api/models/{id}",
            get(get_model_by_id).put(update_model).delete(delete_model),
        )
}
