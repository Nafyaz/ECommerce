use crate::modules::identity::adapters::inbound::http::handlers::{login_handler, register_handler};
use crate::modules::identity::adapters::outbound::auth::Argon2PasswordHasher;
use crate::modules::identity::adapters::outbound::persistence::PgIdentityRepository;
use crate::modules::identity::application::command_services::IdentityCommandService;
use crate::modules::identity::ports::inbound::IdentityCommandPort;
use crate::modules::identity::ports::outbound::{IdentityRepositoryPort, PasswordHasherPort, TokenServicePort};
use axum::Router;
use axum::routing::post;
use sqlx::PgPool;
use std::sync::Arc;

// TODO: should IdentityState be defined in router, or in port / application / root directory of identity module?
#[derive(Clone)]
pub struct IdentityState {
    pub command_service: Arc<dyn IdentityCommandPort>,
    // pub query_service: Arc<dyn IdentityQueryPort>,
}

pub fn create_router(pool: PgPool, token_service: Arc<dyn TokenServicePort>) -> Router {
    let identity_repo: Arc<dyn IdentityRepositoryPort> = Arc::new(PgIdentityRepository::new(pool.clone()));

    let password_hasher: Arc<dyn PasswordHasherPort> = Arc::new(Argon2PasswordHasher);

    let command_service: Arc<dyn IdentityCommandPort> = Arc::new(IdentityCommandService::new(
        identity_repo.clone(),
        password_hasher,
        token_service.clone(),
    ));

    // let query_service: Arc<dyn IdentityQueryPort> = Arc::new(IdentityQueryService::new(identity_repo.clone()));

    let state = IdentityState {
        command_service,
        // query_service,
    };

    let public_router = Router::new()
        .route("/register", post(register_handler::handle))
        .route("/login", post(login_handler::handle));

    Router::new().merge(public_router).with_state(state)
}
