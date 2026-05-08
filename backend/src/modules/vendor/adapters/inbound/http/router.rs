use crate::infrastructure::http::middleware::AuthState;
use crate::infrastructure::http::middleware::auth_middleware::auth_middleware;
use crate::modules::vendor::VendorHttpState;
use crate::modules::vendor::adapters::inbound::http::handlers::create_vendor;
use axum::routing::post;
use axum::{Router, middleware};

pub fn create_router<S>(auth_state: AuthState) -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    VendorHttpState: axum::extract::FromRef<S>,
{
    let protected_router = Router::new()
        .route("/", post(create_vendor::handle))
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware));

    Router::new().merge(protected_router)
}
