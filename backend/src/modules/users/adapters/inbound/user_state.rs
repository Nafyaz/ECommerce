use crate::modules::users::ports::inbound::UserCommandPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub command_service: Arc<dyn UserCommandPort>,
}

impl UserState {
    pub fn build()
}