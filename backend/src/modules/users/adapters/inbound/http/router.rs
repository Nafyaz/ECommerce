use crate::modules::users::adapters::inbound::http::handlers::{create_user_by_email_handler, login_by_email_handler};
use crate::modules::users::adapters::outbound::auth::JwtTokenService;
use crate::modules::users::adapters::outbound::persistence::PgUserRepository;
use crate::modules::users::application::command_services::UserCommandService;
use crate::modules::users::application::query_services::UserQueryService;
use crate::modules::users::ports::inbound::{UserCommandPort, UserQueryPort};
use crate::modules::users::ports::outbound::{TokenServicePort, UserRepositoryPort};
use axum::Router;
use axum::routing::post;
use secrecy::SecretString;
use sqlx::PgPool;
use std::sync::Arc;

// TODO: should UserState be defined in router, or in port / application / root directory of users module?
#[derive(Clone)]
pub struct UserState {
    pub command_service: Arc<dyn UserCommandPort>,
    pub query_service: Arc<dyn UserQueryPort>,
}

// TODO: jwt_secret, expiration_hours should be taken from config
pub fn create_router(pool: PgPool, jwt_secret: SecretString) -> Router {
    let user_repo: Arc<dyn UserRepositoryPort> = Arc::new(PgUserRepository::new(pool.clone()));

    let token_service: Arc<dyn TokenServicePort> = Arc::new(JwtTokenService::new(jwt_secret.clone(), 24));

    let command_service: Arc<dyn UserCommandPort> =
        Arc::new(UserCommandService::new(user_repo.clone(), token_service.clone()));

    let query_service: Arc<dyn UserQueryPort> = Arc::new(UserQueryService::new(user_repo.clone()));

    let state = UserState {
        command_service,
        query_service,
    };

    Router::new()
        .route("/create-user-by-email", post(create_user_by_email_handler::handle))
        // .route("/create-user-by-phone", post(create_user_by_phone_handler))
        .route("/login", post(login_by_email_handler::handle))
        // .route("/get-info", get(get_info_handler))
        .with_state(state)
}
