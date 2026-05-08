use crate::modules::notification::adapters::outbound::LogEmailProvider;
use crate::modules::notification::application::command_services::NotificationCommandService;
use crate::modules::notification::ports::inbound::NotificationCommandPort;
use crate::modules::notification::ports::outbound::EmailProviderPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationState {
    pub command_service: Arc<dyn NotificationCommandPort>,
}

impl NotificationState {
    pub fn new() -> Self {
        let email_provider: Arc<dyn EmailProviderPort> = Arc::new(LogEmailProvider);
        let command_service: Arc<dyn NotificationCommandPort> =
            Arc::new(NotificationCommandService::new(email_provider));
        Self { command_service }
    }
}
