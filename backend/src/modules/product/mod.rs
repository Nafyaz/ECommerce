mod adapters;
mod application;
mod domain;
mod errors;
mod ports;

pub use self::adapters::inbound::http::ProductHttpState;
pub use self::adapters::inbound::http::create_router;
pub(crate) use self::adapters::outbound::identities::ProductIdentityQueryAdapter;
pub use self::adapters::outbound::persistence::PgProductImageRepository;
pub use self::adapters::outbound::persistence::PgProductRepository;
pub use self::adapters::outbound::persistence::R2ObjectStorage;
pub(crate) use self::adapters::outbound::vendors::ProductVendorQueryAdapter;
pub use self::application::command_services::ProductCommandService;
pub use self::application::command_services::ProductImageCommandService;
