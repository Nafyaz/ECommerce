use crate::config::auth::AuthConfig;
use crate::modules::identity::adapters::outbound::auth::Argon2PasswordHasher;
use crate::modules::identity::adapters::outbound::notifications::NotificationModuleAdapter;
use crate::modules::identity::adapters::outbound::otp::OtpServiceAdapter;
use crate::modules::identity::adapters::outbound::persistence::{PgIdentityRepository, PgOtpRepository};
use crate::modules::identity::application::command_services::IdentityCommandService;
use crate::modules::identity::ports::inbound::IdentityCommandPort;
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryPort, NotificationPort, OtpRepositoryPort, OtpServicePort, PasswordHasherPort,
};
use crate::modules::identity::{JwtTokenService, TokenServicePort};
use crate::modules::notifications::NotificationState;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct IdentityState {
    pub command_service: Arc<dyn IdentityCommandPort>,
    // pub query_service: Arc<dyn IdentityQueryPort>,
}

impl IdentityState {
    pub fn build(pool: PgPool, auth_config: AuthConfig) -> Self {
        let identity_repo: Arc<dyn IdentityRepositoryPort> = Arc::new(PgIdentityRepository::new(pool.clone()));

        let notification_service: Arc<dyn NotificationPort> =
            Arc::new(NotificationModuleAdapter::new(NotificationState::build()));

        let otp_service: Arc<dyn OtpServicePort> = Arc::new(OtpServiceAdapter::new(auth_config.otp_secret().clone()));
        let otp_repo: Arc<dyn OtpRepositoryPort> = Arc::new(PgOtpRepository::new(pool.clone()));

        let password_hasher: Arc<dyn PasswordHasherPort> = Arc::new(Argon2PasswordHasher);

        let token_service: Arc<dyn TokenServicePort> = Arc::new(JwtTokenService::new(
            auth_config.jwt_secret().clone(),
            auth_config.access_token_ttl(),
            auth_config.refresh_token_ttl(),
        ));

        let command_service: Arc<dyn IdentityCommandPort> = Arc::new(IdentityCommandService::new(
            identity_repo,
            notification_service,
            otp_service,
            otp_repo,
            password_hasher,
            token_service,
        ));

        // let query_service: Arc<dyn IdentityQueryPort> = Arc::new(IdentityQueryService::new(identity_repo.clone()));

        Self {
            command_service,
            // query_service,
        }
    }
}
