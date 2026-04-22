mod adapters;
mod application;
mod domain;
pub mod errors;
mod ports;

pub use adapters::inbound::http::router::create_router;
