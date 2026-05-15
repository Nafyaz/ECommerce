use crate::AppState;
use crate::config::config::Config;
use crate::infrastructure::http::middleware::AuthState;
use crate::infrastructure::persistence::database::connection_pool::create_pool;
use crate::modules::identity::{
    Argon2PasswordHasher, HmacOtpProvider, IdentityAuthService, IdentityCommandService, IdentityHttpState,
    IdentityQueryService, JwtAuthenticator, JwtTokenProvider, NotificationModuleAdapter, PgIdentityRepository,
    PgOtpRepository,
};
use crate::modules::notification::{LogEmailProvider, NotificationCommandService};
use crate::modules::product::{
    PgProductImageRepository, PgProductRepository, ProductCommandService, ProductHttpState,
    ProductIdentityQueryAdapter, ProductImageCommandService, ProductVendorQueryAdapter, R2ObjectStorage,
};
use crate::modules::user::ports::inbound::UserQueryPort;
use crate::modules::user::{
    PgUserRepository, UserCommandService, UserHttpState, UserIdentityQueryAdapter, UserQueryService,
};
use crate::modules::vendor::{
    PgVendorRepository, VendorCommandService, VendorHttpState, VendorIdentityQueryAdapter, VendorQueryService,
};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use secrecy::ExposeSecret;
use std::error::Error;
use std::sync::Arc;

pub async fn build_app_state(config: &Config) -> Result<AppState, Box<dyn Error>> {
    let db_pool = create_pool(&config.database).await?;
    tracing::info!("Database connection established");

    // Notification
    let email_provider = Arc::new(LogEmailProvider);
    let notification_commands = Arc::new(NotificationCommandService::new(email_provider));

    // Identity
    let identity_repo = Arc::new(PgIdentityRepository::new(db_pool.clone()));
    let otp_repo = Arc::new(PgOtpRepository::new(db_pool.clone()));
    let otp_service = Arc::new(HmacOtpProvider::new(config.auth.otp_secret().clone()));
    let password_hasher = Arc::new(Argon2PasswordHasher);
    let token_service = Arc::new(JwtTokenProvider::new(
        config.auth.jwt_secret().clone(),
        config.auth.access_token_ttl(),
        config.auth.refresh_token_ttl(),
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

    let identity_queries = Arc::new(IdentityQueryService::new(identity_repo.clone()));
    let identity_auth = Arc::new(IdentityAuthService::new(
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
    let product_repo = Arc::new(PgProductRepository::new(db_pool.clone()));
    let product_identity_port = Arc::new(ProductIdentityQueryAdapter::new(identity_queries.clone()));
    let product_vendor_port = Arc::new(ProductVendorQueryAdapter::new(vendor_queries.clone()));

    let product_commands = Arc::new(ProductCommandService::new(
        product_identity_port.clone(),
        product_vendor_port.clone(),
        product_repo.clone(),
    ));

    let product_image_repo = Arc::new(PgProductImageRepository::new(db_pool.clone()));

    let credentials = Credentials::new(
        config.storage.access_key_id().expose_secret(),
        config.storage.secret_access_key().expose_secret(),
        None,
        None,
        "r2",
    );

    let s3_config = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(config.storage.region().to_owned()))
        .credentials_provider(credentials)
        .endpoint_url(config.storage.endpoint())
        .build();

    let object_storage = Arc::new(R2ObjectStorage::new(
        Client::from_conf(s3_config),
        config.storage.bucket_name().to_owned(),
        config.storage.presigned_url_expiry(),
    ));

    let product_image_commands = Arc::new(ProductImageCommandService::new(
        product_identity_port.clone(),
        product_vendor_port.clone(),
        product_repo.clone(),
        product_image_repo.clone(),
        object_storage,
    ));

    // let product_queries = Arc::new(ProductQueryService::new(product_repo));

    Ok(AppState {
        auth_state: AuthState::new(authenticator),
        identity_http_state: IdentityHttpState::new(identity_auth, identity_commands, identity_queries),
        user_http_state: UserHttpState::new(user_commands, user_queries),
        vendor_http_state: VendorHttpState::new(vendor_commands, vendor_queries),
        product_http_state: ProductHttpState::new(product_commands, product_image_commands),
    })
}
