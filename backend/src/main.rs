use axum::Router;
use axum::http::StatusCode;
use backend::build_app_state;
use backend::config::config::Config;
use backend::infrastructure::persistence::database::connection_pool::create_pool;
use backend::modules::{identity, product, user, vendor};
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

    tracing::info!("Starting multi-vendor ecommerce backend");

    let config = Config::load()?;
    tracing::info!("Configuration loaded");

    let app_state = build_app_state(&config).await?;

    let app = Router::new()
        .nest("/v1/identities", identity::create_router())
        .nest("/v1/users", user::create_router(app_state.auth_state.clone()))
        .nest("/v1/vendors", vendor::create_router(app_state.auth_state.clone()))
        .nest("/v1/products", product::create_router(app_state.auth_state.clone()))
        .fallback(|| async { (StatusCode::NOT_FOUND, "Not found") })
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server starting on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
