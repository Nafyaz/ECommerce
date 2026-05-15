use crate::modules::identity::ports::inbound::{IdentityAuthPort, IdentityCommandPort, IdentityQueryPort};
use std::sync::Arc;

#[derive(Clone)]
pub struct IdentityHttpState {
    pub auth_service: Arc<dyn IdentityAuthPort>,
    pub command_service: Arc<dyn IdentityCommandPort>,
    pub query_service: Arc<dyn IdentityQueryPort>,
}

impl IdentityHttpState {
    pub fn new(
        auth_service: Arc<dyn IdentityAuthPort>,
        command_service: Arc<dyn IdentityCommandPort>,
        query_service: Arc<dyn IdentityQueryPort>,
    ) -> Self {
        Self {
            auth_service,
            command_service,
            query_service,
        }
    }
}
