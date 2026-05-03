use crate::modules::vendors::ports::inbound::VendorCommandPort;
use std::sync::Arc;

#[derive(Clone)]
pub struct VendorState {
    pub command_service: Arc<dyn VendorCommandPort>,
}
