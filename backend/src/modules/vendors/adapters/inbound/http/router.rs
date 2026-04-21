use crate::infrastructure::http::middleware::AuthState;
use crate::infrastructure::http::middleware::auth_middleware::auth_middleware;
use crate::modules::identity::TokenServicePort;
use crate::modules::vendors::ports::inbound::VendorCommandPort;
use crate::modules::vendors::ports::outbound::VendorRepositoryPort;
use axum::routing::post;
use axum::{Router, middleware};
use sqlx::PgPool;
use std::sync::Arc;

pub struct VendorState {
    pub command_service: Arc<dyn VendorCommandPort>,
    // pub query_service: Arc<dyn VendorQueryPort>,
}

pub fn create_router(pool: PgPool, token_service: Arc<dyn TokenServicePort>) -> Router {
    let vendor_repo: Arc<dyn VendorRepositoryPort> = Arc::new(PgVendorRepository::new(pool.clone()));

    let auth_state = AuthState {
        token_service: token_service.clone(),
    };

    let protected_router = Router::new()
        .route("/create-vendor", post(create_vendor::handle))
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware));

    Router::new().merge(protected_router)
}
