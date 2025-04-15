#![allow(unused_imports)]
#![allow(dead_code)]

mod api;
mod utils;
mod models;
mod services;
mod prelude;

use prelude::*;
use tracing::{info, warn};
use dotenvy::dotenv;
use axum::Router;
use sqlx::PgPool;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment variables and logger
    if let Err(e) = dotenv() { warn!("Failed to load .env file: {}", e); }
    logger::init();

    // Database connection
    let db_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env".to_string())?;
    let pool = PgPool::connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // Build Axum router
    let config = Config::from_env()?;
    let app = Router::new()
        .merge(app_routes())
        .with_state(pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Start server
    info!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
