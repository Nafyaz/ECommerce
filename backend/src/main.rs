use axum::Router;
use backend::infrastructure::config::config::Config;
use backend::infrastructure::database::connection_pool::create_pool;
use backend::modules::users;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

// TODO: Analyze every possible '?', their types and how they are getting handled
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting multi-vendors ecommerce backend");

    let config = Config::load()?;
    tracing::info!("Configuration loaded");

    let db_pool = create_pool(&config.database).await?;
    tracing::info!("Database connection established");

    let app = Router::new()
        .merge(users::create_router(db_pool, config.database.url().clone()))
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server starting on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
