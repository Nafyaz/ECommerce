mod adapters;
mod application;
mod domain;
mod errors;
pub mod ports;

pub use adapters::inbound::http::UserHttpState;
pub(crate) use adapters::outbound::identities::IdentityModuleAdapter;
pub(crate) use adapters::outbound::persistence::PgUserRepository;
pub(crate) use application::command_services::UserCommandService;
pub(crate) use application::query_services::UserQueryService;
