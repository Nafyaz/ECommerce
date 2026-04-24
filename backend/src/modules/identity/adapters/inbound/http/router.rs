use crate::modules::identity::adapters::inbound::http::handlers::{
    create_user_by_email_handler, login_by_email_handler,
};
use crate::modules::identity::adapters::outbound::auth::Argon2PasswordHasher;
use crate::modules::identity::adapters::outbound::persistence::PgUserRepository;
use crate::modules::identity::application::command_services::UserCommandService;
use crate::modules::identity::application::query_services::UserQueryService;
use crate::modules::identity::ports::inbound::{UserCommandPort, UserQueryPort};
use crate::modules::identity::ports::outbound::{PasswordHasherPort, TokenServicePort, UserRepositoryPort};
use axum::Router;
use axum::routing::post;
use sqlx::PgPool;
use std::sync::Arc;

// TODO: should UserState be defined in router, or in port / application / root directory of identity module?
#[derive(Clone)]
pub struct UserState {
    pub command_service: Arc<dyn UserCommandPort>,
    pub query_service: Arc<dyn UserQueryPort>,
}

pub fn create_router(pool: PgPool, token_service: Arc<dyn TokenServicePort>) -> Router {
    let user_repo: Arc<dyn UserRepositoryPort> = Arc::new(PgUserRepository::new(pool.clone()));

    let password_hasher: Arc<dyn PasswordHasherPort> = Arc::new(Argon2PasswordHasher);

    let command_service: Arc<dyn UserCommandPort> = Arc::new(UserCommandService::new(
        user_repo.clone(),
        password_hasher,
        token_service.clone(),
    ));

    let query_service: Arc<dyn UserQueryPort> = Arc::new(UserQueryService::new(user_repo.clone()));

    let state = UserState {
        command_service,
        query_service,
    };

    // TODO: Should I make separate endpoints or unified for create user by email / phone??
    let public_router = Router::new()
        .route("/create-user-by-email", post(create_user_by_email_handler::handle))
        // .route("/create-user-by-phone", post(create_user_by_phone_handler))
        .route("/login-by-email", post(login_by_email_handler::handle));

    Router::new().merge(public_router).with_state(state)
}
