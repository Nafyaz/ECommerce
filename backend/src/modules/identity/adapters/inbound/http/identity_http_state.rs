use crate::modules::identity::ports::inbound::{IdentityCommandPort, IdentityQueryPort};
use std::sync::Arc;

#[derive(Clone)]
pub struct IdentityHttpState {
    pub command_service: Arc<dyn IdentityCommandPort>,
    pub query_service: Arc<dyn IdentityQueryPort>,
}

impl IdentityHttpState {
    pub fn new(command_service: Arc<dyn IdentityCommandPort>, query_service: Arc<dyn IdentityQueryPort>) -> Self {
        Self {
            command_service,
            query_service,
        }
    }
}
