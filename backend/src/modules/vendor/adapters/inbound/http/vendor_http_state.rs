use crate::modules::vendor::ports::inbound::{VendorCommandPort, VendorQueryPort};
use std::sync::Arc;

#[derive(Clone)]
pub struct VendorHttpState {
    pub command_service: Arc<dyn VendorCommandPort>,
    pub query_service: Arc<dyn VendorQueryPort>,
}

impl VendorHttpState {
    pub fn new(command_service: Arc<dyn VendorCommandPort>, query_service: Arc<dyn VendorQueryPort>) -> Self {
        Self {
            command_service,
            query_service,
        }
    }
}
