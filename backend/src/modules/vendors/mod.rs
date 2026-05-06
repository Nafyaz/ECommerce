mod adapters;
mod application;
mod domain;
mod errors;
pub mod ports;

pub use adapters::inbound::http::VendorHttpState;
pub use adapters::inbound::http::router::create_router;
pub(crate) use adapters::outbound::identities::IdentityQueryAdapter;
pub(crate) use adapters::outbound::persistence::PgVendorRepository;
pub use application::command_services::VendorCommandService;
pub use application::query_services::VendorQueryService;
pub use domain::value_objects::{OwnerId, VendorId};
