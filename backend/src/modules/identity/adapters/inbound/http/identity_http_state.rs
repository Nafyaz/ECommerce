use crate::config::auth::AuthConfig;
use crate::modules::identity::adapters::outbound::auth::Argon2PasswordHasher;
use crate::modules::identity::adapters::outbound::notifications::NotificationModuleAdapter;
use crate::modules::identity::adapters::outbound::otp::OtpServiceAdapter;
use crate::modules::identity::adapters::outbound::persistence::{PgIdentityRepository, PgOtpRepository};
use crate::modules::identity::application::command_services::{IdentityCommandService, IdentityQueryService};
use crate::modules::identity::ports::inbound::{IdentityCommandPort, IdentityQueryPort};
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryPort, NotificationPort, OtpRepositoryPort, OtpServicePort, PasswordHasherPort,
};
use crate::modules::identity::{JwtTokenService, TokenServicePort};
use crate::modules::notifications::NotificationState;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct IdentityHttpState {
    pub command_service: Arc<dyn IdentityCommandPort>,
    pub query_service: Arc<dyn IdentityQueryPort>,
}

impl IdentityHttpState {
    pub fn new(pool: PgPool, auth_config: AuthConfig, notification_state: NotificationState) -> Self {
        let identity_repo = Arc::new(PgIdentityRepository::new(pool.clone()));

        let notification_service = Arc::new(NotificationModuleAdapter::new(notification_state));

        let otp_service = Arc::new(OtpServiceAdapter::new(auth_config.otp_secret().clone()));
        let otp_repo = Arc::new(PgOtpRepository::new(pool.clone()));

        let password_hasher = Arc::new(Argon2PasswordHasher);

        let token_service = Arc::new(JwtTokenService::new(
            auth_config.jwt_secret().clone(),
            auth_config.access_token_ttl(),
            auth_config.refresh_token_ttl(),
        ));

        let command_service = Arc::new(IdentityCommandService::new(
            identity_repo.clone(),
            notification_service,
            otp_service,
            otp_repo,
            password_hasher.clone(),
            token_service.clone(),
        ));

        let query_service = Arc::new(IdentityQueryService::new(identity_repo, password_hasher, token_service));

        Self {
            command_service,
            query_service,
        }
    }
}
