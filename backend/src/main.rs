use axum::Router;
use backend::infrastructure::config::config::Config;
// use backend::infrastructure::database::connection::create_pool;
use backend::modules::users;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    // tracing::info!("Configuration loaded");
    //
    // tracing_subscriber::fmt()
    //     .with_env_filter("multi_vendor_ecommerce=debug,tower_http=debug")
    //     .init();

    // tracing::info!("Starting multi-vendors ecommerce backend");

    // let _db_pool = create_pool(&settings.database).await?;
    // tracing::info!("Database connection established");

    // TODO: Initialize application (legacy) state

    // TODO: Start HTTP server
    let app_state = backend::modules::users::adapters::http::app_state::AppState::new();
    let app = Router::new().merge(users::adapters::http::routes::create_router(app_state));
    tokio::net::TcpListener::bind(&config.server.host).await?;

    // tracing::info!(
    //     "Server starting on {}:{}",
    //     settings.server.host,
    //     settings.server.port
    // );

    Ok(())
}
