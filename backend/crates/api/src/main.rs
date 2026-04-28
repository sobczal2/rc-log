mod asset_paths;
mod auth;
mod config;
mod error;
mod extractors;
mod jwt;
mod maneuver;
mod model;
mod session;
mod state;
mod training_program;
mod user;

use axum::extract::DefaultBodyLimit;
use axum::{Router, serve};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use crate::asset_paths::router::asset_paths_router;
use crate::auth::router::auth_router;
use crate::config::AppConfig;
use crate::maneuver::router::maneuver_router;
use crate::model::router::model_router;
use crate::session::router::session_router;
use crate::training_program::router::training_program_router;
use crate::state::ResolverCacheConfig;
use crate::user::router::user_router;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file");

    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = AppConfig::load();

    info!(environment = ?config.environment, "Starting server");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    info!("Database connection pool established");

    let state = AppState::new(
        pool,
        config.jwt_secret.clone(),
        ResolverCacheConfig {
            model_ttl_seconds: config.model_cache_ttl_seconds,
            model_size: config.model_cache_size,
            maneuver_ttl_seconds: config.maneuver_cache_ttl_seconds,
            maneuver_size: config.maneuver_cache_size,
            variation_ttl_seconds: config.variation_cache_ttl_seconds,
            variation_size: config.variation_cache_size,
            video_ttl_seconds: config.video_cache_ttl_seconds,
            video_size: config.video_cache_size,
            photo_ttl_seconds: config.photo_cache_ttl_seconds,
            photo_size: config.photo_cache_size,
        },
        config.asset_path.clone(),
    );

    let app = Router::new()
        .merge(maneuver_router())
        .merge(model_router())
        .merge(session_router())
        .merge(training_program_router())
        .merge(auth_router())
        .merge(user_router())
        .merge(asset_paths_router())
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .with_state(state)
        .nest_service("/api/assets", ServeDir::new(config.asset_path.clone()));

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!(address = %addr, "Listening");

    serve(listener, app).await.expect("Server error");
}
