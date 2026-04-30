use crate::modules::notifications::adapters::outbound::LogEmailProvider;
use crate::modules::notifications::application::command_services::NotificationCommandService;
use crate::modules::notifications::ports::inbound::NotificationCommandPort;
use crate::modules::notifications::ports::outbound::EmailProviderPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct NotificationState {
    pub command_service: Arc<dyn NotificationCommandPort>,
}

impl NotificationState {
    pub fn build() -> Self {
        let email_provider: Arc<dyn EmailProviderPort> = Arc::new(LogEmailProvider);
        let command_service: Arc<dyn NotificationCommandPort> =
            Arc::new(NotificationCommandService::new(email_provider));
        Self { command_service }
    }
}
