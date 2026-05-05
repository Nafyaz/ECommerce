use crate::AppState;
use crate::config::auth::AuthConfig;
use crate::modules::identity::ports::inbound::{IdentityCommandPort, IdentityQueryPort};
use crate::modules::identity::ports::outbound::{
    IdentityRepositoryPort, NotificationPort, OtpRepositoryPort, OtpServicePort, PasswordHasherPort, TokenServicePort,
};
use crate::modules::identity::{
    Argon2PasswordHasher, IdentityCommandService, IdentityHttpState, IdentityQueryService, JwtTokenService,
    NotificationModuleAdapter, OtpServiceAdapter, PgIdentityRepository, PgOtpRepository,
};
use crate::modules::notifications::{
    EmailProviderPort, LogEmailProvider, NotificationCommandPort, NotificationCommandService,
};
use crate::modules::users::ports::inbound::{UserCommandPort, UserQueryPort};
use crate::modules::users::ports::outbound::{IdentityPort as UserIdentityPort, UserRepositoryPort};
use crate::modules::users::{
    IdentityModuleAdapter as UserIdentityModuleAdapter, PgUserRepository, UserCommandService, UserHttpState,
    UserQueryService,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn build_app_state(db_pool: PgPool, auth_config: AuthConfig) -> AppState {
    let email_provider: Arc<dyn EmailProviderPort> = Arc::new(LogEmailProvider);
    let notification_commands: Arc<dyn NotificationCommandPort> =
        Arc::new(NotificationCommandService::new(email_provider));

    let identity_repo: Arc<dyn IdentityRepositoryPort> = Arc::new(PgIdentityRepository::new(db_pool.clone()));
    let otp_repo: Arc<dyn OtpRepositoryPort> = Arc::new(PgOtpRepository::new(db_pool.clone()));
    let otp_service: Arc<dyn OtpServicePort> = Arc::new(OtpServiceAdapter::new(auth_config.otp_secret().clone()));
    let password_hasher: Arc<dyn PasswordHasherPort> = Arc::new(Argon2PasswordHasher);
    let token_service: Arc<dyn TokenServicePort> = Arc::new(JwtTokenService::new(
        auth_config.jwt_secret().clone(),
        auth_config.access_token_ttl(),
        auth_config.refresh_token_ttl(),
    ));
    let notification_port: Arc<dyn NotificationPort> = Arc::new(NotificationModuleAdapter::new(notification_commands));

    let identity_commands: Arc<dyn IdentityCommandPort> = Arc::new(IdentityCommandService::new(
        identity_repo.clone(),
        notification_port,
        otp_service,
        otp_repo,
        password_hasher.clone(),
        token_service.clone(),
    ));

    let identity_queries: Arc<dyn IdentityQueryPort> =
        Arc::new(IdentityQueryService::new(identity_repo, password_hasher, token_service));

    let user_repo: Arc<dyn UserRepositoryPort> = Arc::new(PgUserRepository::new(db_pool));
    let user_identity_port: Arc<dyn UserIdentityPort> =
        Arc::new(UserIdentityModuleAdapter::new(identity_queries.clone()));
    let user_commands: Arc<dyn UserCommandPort> =
        Arc::new(UserCommandService::new(user_identity_port, user_repo.clone()));
    let user_queries: Arc<dyn UserQueryPort> = Arc::new(UserQueryService::new(user_repo));

    AppState {
        identity_http_state: IdentityHttpState::new(identity_commands, identity_queries),
        user_http_state: UserHttpState::new(user_commands, user_queries),
    }
}
