use crate::modules::users::adapters::outbound::persistence::PgUserRepository;
use crate::modules::users::application::command_services::UserCommandService;
use crate::modules::users::ports::inbound::UserCommandPort;
use crate::modules::users::ports::outbound::UserRepositoryPort;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub command_service: Arc<dyn UserCommandPort>,
}

impl UserState {
    pub fn build(pool: PgPool) -> Self {
        let user_repo: Arc<dyn UserRepositoryPort> = Arc::new(PgUserRepository::new(pool));

        let command_service: Arc<dyn UserCommandPort> = Arc::new(UserCommandService::new(user_repo));

        Self { command_service }
    }
}
