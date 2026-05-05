use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::vendors::adapters::outbound::identities::IdentityModuleAdapter;
use crate::modules::vendors::adapters::outbound::persistence::PgVendorRepository;
use crate::modules::vendors::application::command_services::VendorCommandService;
use crate::modules::vendors::ports::inbound::VendorCommandPort;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct VendorHttpState {
    pub command_service: Arc<dyn VendorCommandPort>,
}

impl VendorHttpState {
    pub fn build(identity_queries: Arc<dyn IdentityQueryPort>, db_pool: PgPool) -> Self {
        let identity_service = Arc::new(IdentityModuleAdapter::new(identity_queries.clone()));

        let vendor_repo = Arc::new(PgVendorRepository::new(db_pool.clone()));

        let command_service = Arc::new(VendorCommandService::new(identity_service, vendor_repo.clone()));

        Self { command_service }
    }
}
