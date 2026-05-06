mod adapters;
mod application;
mod domain;
mod errors;
mod ports;

pub use self::adapters::inbound::http::ProductHttpState;
pub use self::adapters::inbound::http::create_router;
