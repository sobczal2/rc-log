mod config;
mod error;
mod maneuver;
mod state;

use sqlx::postgres::PgPoolOptions;
use state::AppState;

use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    let config = AppConfig::load();

    println!("Starting in {:?} mode", config.environment);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    let state = AppState::new(pool);

    let app = axum::Router::new().merge(maneuver::router::maneuver_router()).with_state(state);

    let addr = config.socket_addr();
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind to address");

    println!("Listening on http://{}", addr);

    axum::serve(listener, app).await.expect("Server error");
}
