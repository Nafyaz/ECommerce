use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::vendors::adapters::outbound::identities::IdentityModuleAdapter;
use crate::modules::vendors::adapters::outbound::persistence::PgVendorRepository;
use crate::modules::vendors::application::command_services::VendorCommandService;
use crate::modules::vendors::ports::inbound::{VendorCommandPort, VendorQueryPort};
use sqlx::PgPool;
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
