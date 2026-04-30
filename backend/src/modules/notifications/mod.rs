mod adapters;
mod application;
mod domain;
mod errors;
mod ports;

pub use adapters::inbound::http::NotificationState;
pub use application::commands::SendEmailCommand;
pub use errors::NotificationError;
