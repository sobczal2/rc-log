mod config;
mod error;
mod maneuver;
mod extractors;
mod state;

use axum::{serve, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::AppConfig;
use crate::maneuver::router::maneuver_router;

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

    let state = AppState::new(pool);

    let app = Router::new().merge(maneuver_router()).with_state(state);

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!(address = %addr, "Listening");

    serve(listener, app).await.expect("Server error");
}
