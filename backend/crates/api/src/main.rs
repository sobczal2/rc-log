mod asset_paths;
mod auth;
mod config;
mod error;
mod extractors;
mod jwt;
mod maneuver;
mod state;
mod user;

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

    let state = AppState::new(pool, config.jwt_secret.clone(), config.asset_cache_size);

    let app = Router::new()
        .merge(maneuver_router())
        .merge(auth_router())
        .merge(user_router())
        .merge(asset_paths_router())
        .with_state(state)
        .nest_service("/api/assets", ServeDir::new(config.asset_path.clone()));

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!(address = %addr, "Listening");

    serve(listener, app).await.expect("Server error");
}
