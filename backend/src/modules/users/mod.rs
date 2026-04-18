mod adapters;
mod application;
mod domain;
mod ports;

pub use adapters::inbound::http::router::create_router;
