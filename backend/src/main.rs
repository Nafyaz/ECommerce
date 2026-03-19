use backend::infrastructure::config::settings::Settings;
use backend::infrastructure::database::connection::create_pool;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::load()?;
    tracing::info!("Configuration loaded");

    tracing_subscriber::fmt()
        .with_env_filter("multi_vendor_ecommerce=debug,tower_http=debug")
        .init();

    tracing::info!("Starting multi-vendors ecommerce backend");

    let _db_pool = create_pool(&settings.database).await?;
    tracing::info!("Database connection established");

    // TODO: Initialize application (legacy) state
    // TODO: Start HTTP server

    tracing::info!(
        "Server starting on {}:{}",
        settings.server.host,
        settings.server.port
    );

    Ok(())
}
