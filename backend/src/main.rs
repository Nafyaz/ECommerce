use crate::config::settings::AppConfig;
use crate::infrastructure::database::connection::create_pool;
use tracing_subscriber;

mod api;
mod application;
mod config;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::load()?;
    tracing::info!("Configuration loaded");

    tracing_subscriber::fmt()
        .with_env_filter("multi_vendor_ecommerce=debug,tower_http=debug")
        .init();

    tracing::info!("Starting multi-vendor ecommerce backend");

    let _db_pool = create_pool(&app_config.database).await?;
    tracing::info!("Database connection established");

    // TODO: Initialize application state
    // TODO: Start HTTP server

    tracing::info!(
        "Server starting on {}:{}",
        app_config.server.host,
        app_config.server.port
    );

    Ok(())
}
