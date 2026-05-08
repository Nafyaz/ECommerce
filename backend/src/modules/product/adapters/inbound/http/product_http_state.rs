use crate::modules::product::ports::inbound::ProductCommandPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductHttpState {
    pub command_port: Arc<dyn ProductCommandPort>,
}

impl ProductHttpState {
    pub fn new(command_port: Arc<dyn ProductCommandPort>) -> Self {
        Self { command_port }
    }
}
