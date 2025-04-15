#![allow(unused_imports)]
#![allow(dead_code)]

mod api;
mod utils;
mod models;
mod prelude;

use prelude::*;
use tracing::info;
use dotenvy::dotenv;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize environment variables and logger
    dotenv().ok();
    logger::init();

    // Build Axum router
    let config = Config::from_env()?;
    let app = Router::new().merge(app_routes());
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Start server
    info!("Server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
