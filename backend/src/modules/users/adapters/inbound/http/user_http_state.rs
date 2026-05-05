use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::users::adapters::outbound::identities::IdentityModuleAdapter;
use crate::modules::users::adapters::outbound::persistence::PgUserRepository;
use crate::modules::users::application::command_services::UserCommandService;
use crate::modules::users::ports::inbound::UserCommandPort;
use crate::modules::users::ports::outbound::UserRepositoryPort;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserHttpState {
    pub command_service: Arc<dyn UserCommandPort>,
}

impl UserHttpState {
    pub fn new(pool: PgPool, identity_queries: Arc<dyn IdentityQueryPort>) -> Self {
        let identity_service = Arc::new(IdentityModuleAdapter::new(identity_queries.clone()));
        let user_repo: Arc<dyn UserRepositoryPort> = Arc::new(PgUserRepository::new(pool));

        let command_service: Arc<dyn UserCommandPort> = Arc::new(UserCommandService::new(identity_service, user_repo));

        Self { command_service }
    }
}
