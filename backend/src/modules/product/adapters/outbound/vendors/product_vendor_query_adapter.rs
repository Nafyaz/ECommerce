use crate::modules::product::domain::value_objects::{ProductActorId, SupplierId};
use crate::modules::product::ports::outbound::{ProductVendorPort, ProductVendorPortError};
use crate::modules::vendor::ports::inbound::VendorQueryPort;
use crate::modules::vendor::{OwnerId, VendorDomainError, VendorId};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductVendorQueryAdapter {
    vendor_queries: Arc<dyn VendorQueryPort>,
}

impl ProductVendorQueryAdapter {
    pub fn new(vendor_queries: Arc<dyn VendorQueryPort>) -> Self {
        Self { vendor_queries }
    }
}

#[async_trait]
impl ProductVendorPort for ProductVendorQueryAdapter {
    async fn check_ownership(
        &self,
        supplier_id: SupplierId,
        actor_id: ProductActorId,
    ) -> Result<bool, ProductVendorPortError> {
        let vendor_id = VendorId::from_uuid(supplier_id.as_uuid());
        let owner_id = OwnerId::from_uuid(actor_id.as_uuid());

        self.vendor_queries
            .check_ownership(vendor_id, owner_id)
            .await
            .map_err(|error| match error {
                VendorDomainError::VendorNotFound => ProductVendorPortError::NotFound,
                VendorDomainError::InternalError(_) => ProductVendorPortError::Unavailable,
                _ => ProductVendorPortError::Unexpected,
            })
    }
}
