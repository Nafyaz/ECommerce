use crate::infrastructure::http::middleware::AuthState;
use crate::infrastructure::http::middleware::auth_middleware::auth_middleware;
use crate::modules::identity::TokenServicePort;
use crate::modules::users::ports::inbound::user_query_port::UserQueryPort;
use crate::modules::vendors::adapters::inbound::http::handler::create_vendor;
use crate::modules::vendors::adapters::outbound::persistence::PgVendorRepository;
use crate::modules::vendors::application::command_services::VendorCommandService;
use crate::modules::vendors::ports::inbound::VendorCommandPort;
use crate::modules::vendors::ports::outbound::VendorRepositoryPort;
use axum::routing::post;
use axum::{Router, middleware};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct VendorState {
    pub command_service: Arc<dyn VendorCommandPort>,
    // pub query_service: Arc<dyn VendorQueryPort>,
    pub user_query_service: Arc<dyn UserQueryPort>,
}

pub fn create_router(pool: PgPool, token_service: Arc<dyn TokenServicePort>) -> Router {
    let vendor_repo: Arc<dyn VendorRepositoryPort> = Arc::new(PgVendorRepository::new(pool.clone()));
    let command_service = Arc::new(VendorCommandService::new(vendor_repo.clone()));
    let vendor_state = VendorState { command_service };

    let auth_state = AuthState {
        token_service: token_service.clone(),
    };

    let protected_router = Router::new()
        .route("/", post(create_vendor::handle))
        .layer(middleware::from_fn_with_state(auth_state, auth_middleware))
        .with_state(vendor_state);

    Router::new().merge(protected_router)
}
