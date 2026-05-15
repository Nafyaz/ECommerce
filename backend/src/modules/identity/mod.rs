mod adapters;
mod application;
mod domain;
pub mod ports;

// TODO: I still don't understand pub vs pub(crate) ToT
pub use adapters::inbound::http::IdentityHttpState;
pub use adapters::inbound::http::router::create_router;
pub(crate) use adapters::outbound::auth::Argon2PasswordHasher;
pub(crate) use adapters::outbound::auth::JwtAuthenticator;
pub use adapters::outbound::auth::JwtTokenProvider;
pub(crate) use adapters::outbound::notifications::NotificationModuleAdapter;
pub(crate) use adapters::outbound::otp::HmacOtpProvider;
pub(crate) use adapters::outbound::persistence::{PgIdentityRepository, PgOtpRepository};
pub(crate) use application::IdentityAppError;
pub(crate) use application::services::IdentityAuthService;
pub(crate) use application::services::IdentityCommandService;
pub(crate) use application::services::IdentityQueryService;
pub use domain::value_objects::IdentityId;
pub use ports::outbound::TokenProviderError;
pub use ports::outbound::TokenProviderPort;
