use crate::infrastructure::http::middleware::AuthState;
use crate::infrastructure::http::middleware::auth_middleware::auth_middleware;
use crate::modules::product::adapters::inbound::http::ProductHttpState;
use crate::modules::product::adapters::inbound::http::handlers::create_product_handler;
use axum::routing::post;
use axum::{Router, middleware};

pub fn create_router<S>(auth_state: AuthState) -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    ProductHttpState: axum::extract::FromRef<S>,
{
    let protected_router = Router::new()
        .route("/", post(create_product_handler::handle))
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware));

    Router::new().merge(protected_router)
}
