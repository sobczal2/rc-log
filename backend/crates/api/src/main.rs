mod config;
mod error;
mod maneuver;
mod shared;
mod state;

use sqlx::postgres::PgPoolOptions;
use state::AppState;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = AppConfig::load();

    info!(environment = ?config.environment, "Starting server");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    info!("Database connection pool established");

    let state = AppState::new(pool);

    let app = axum::Router::new().merge(maneuver::router::maneuver_router()).with_state(state);

    let addr = config.socket_addr();
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    info!(address = %addr, "Listening");

    axum::serve(listener, app).await.expect("Server error");
}
