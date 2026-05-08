use crate::AppState;
use crate::config::auth::AuthConfig;
use crate::infrastructure::http::middleware::AuthState;
use crate::modules::identity::{
    Argon2PasswordHasher, HmacOtpProvider, IdentityCommandService, IdentityHttpState, IdentityQueryService,
    JwtAuthenticator, JwtTokenProvider, NotificationModuleAdapter, PgIdentityRepository, PgOtpRepository,
};
use crate::modules::notifications::{LogEmailProvider, NotificationCommandService};
use crate::modules::products::{
    PgProductRepository, ProductCommandService, ProductHttpState, ProductIdentityQueryAdapter,
};
use crate::modules::users::ports::inbound::UserQueryPort;
use crate::modules::users::{
    PgUserRepository, UserCommandService, UserHttpState, UserIdentityQueryAdapter, UserQueryService,
};
use crate::modules::vendors::{
    PgVendorRepository, VendorCommandService, VendorHttpState, VendorIdentityQueryAdapter, VendorQueryService,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn build_app_state(db_pool: PgPool, auth_config: AuthConfig) -> AppState {
    // Notification
    let email_provider = Arc::new(LogEmailProvider);
    let notification_commands = Arc::new(NotificationCommandService::new(email_provider));

    // Identity
    let identity_repo = Arc::new(PgIdentityRepository::new(db_pool.clone()));
    let otp_repo = Arc::new(PgOtpRepository::new(db_pool.clone()));
    let otp_service = Arc::new(HmacOtpProvider::new(auth_config.otp_secret().clone()));
    let password_hasher = Arc::new(Argon2PasswordHasher);
    let token_service = Arc::new(JwtTokenProvider::new(
        auth_config.jwt_secret().clone(),
        auth_config.access_token_ttl(),
        auth_config.refresh_token_ttl(),
    ));
    let notification_port = Arc::new(NotificationModuleAdapter::new(notification_commands));

    let identity_commands = Arc::new(IdentityCommandService::new(
        identity_repo.clone(),
        notification_port,
        otp_service,
        otp_repo,
        password_hasher.clone(),
        token_service.clone(),
    ));

    let identity_queries = Arc::new(IdentityQueryService::new(
        identity_repo,
        password_hasher,
        token_service.clone(),
    ));

    // Auth
    let authenticator = Arc::new(JwtAuthenticator::new(token_service.clone()));

    // User
    let user_repo = Arc::new(PgUserRepository::new(db_pool.clone()));
    let user_identity_port = Arc::new(UserIdentityQueryAdapter::new(identity_queries.clone()));
    let user_commands = Arc::new(UserCommandService::new(user_identity_port, user_repo.clone()));
    let user_queries: Arc<dyn UserQueryPort> = Arc::new(UserQueryService::new(user_repo));

    // Vendor
    let vendor_repo = Arc::new(PgVendorRepository::new(db_pool.clone()));
    let vendor_identity_port = Arc::new(VendorIdentityQueryAdapter::new(identity_queries.clone()));

    let vendor_commands = Arc::new(VendorCommandService::new(vendor_identity_port, vendor_repo.clone()));
    let vendor_queries = Arc::new(VendorQueryService::new(vendor_repo));

    // Product
    let product_repo = Arc::new(PgProductRepository::new(db_pool));
    let product_identity_port = Arc::new(ProductIdentityQueryAdapter::new(identity_queries.clone()));

    let product_commands = Arc::new(ProductCommandService::new(product_identity_port, product_repo.clone()));
    // let product_queries = Arc::new(ProductQueryService::new(product_repo));

    AppState {
        auth_state: AuthState::new(authenticator),
        identity_http_state: IdentityHttpState::new(identity_commands, identity_queries),
        user_http_state: UserHttpState::new(user_commands, user_queries),
        vendor_http_state: VendorHttpState::new(vendor_commands, vendor_queries),
        product_http_state: ProductHttpState::new(product_commands),
    }
}
