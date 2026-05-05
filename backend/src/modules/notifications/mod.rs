mod adapters;
mod application;
mod domain;
mod errors;
mod ports;

pub use adapters::inbound::http::NotificationState;
pub(crate) use adapters::outbound::LogEmailProvider;
pub(crate) use application::command_services::NotificationCommandService;
pub use application::commands::SendEmailCommand;
pub use errors::NotificationError;
pub use ports::inbound::NotificationCommandPort;
pub(crate) use ports::outbound::EmailProviderPort;
