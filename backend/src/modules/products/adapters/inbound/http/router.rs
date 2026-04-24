use crate::modules::products::ports::inbound::ProductCommandPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct ProductState {
    pub command_port: Arc<dyn ProductCommandPort>,
}
