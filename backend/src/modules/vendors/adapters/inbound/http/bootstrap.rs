use crate::modules::vendors::adapters::inbound::http::VendorState;
use crate::modules::vendors::adapters::outbound::persistence::PgVendorRepository;
use crate::modules::vendors::application::command_services::VendorCommandService;
use sqlx::PgPool;
use std::sync::Arc;

pub fn build_vendor_state(db_pool: PgPool) -> VendorState {
    let vendor_repo = Arc::new(PgVendorRepository::new(db_pool.clone()));

    let command_service = Arc::new(VendorCommandService::new(vendor_repo.clone()));

    VendorState { command_service }
}
