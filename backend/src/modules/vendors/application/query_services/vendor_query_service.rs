use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::{OwnerId, VendorId};
use crate::modules::vendors::errors::VendorDomainError;
use crate::modules::vendors::ports::inbound::VendorQueryPort;
use crate::modules::vendors::ports::outbound::VendorRepositoryPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct VendorQueryService {
    vendor_repo: Arc<dyn VendorRepositoryPort>,
}

impl VendorQueryService {
    pub fn new(vendor_repo: Arc<dyn VendorRepositoryPort>) -> Self {
        Self { vendor_repo }
    }
}

#[async_trait]
impl VendorQueryPort for VendorQueryService {
    async fn check_ownership(&self, id: &VendorId, owner_id: &OwnerId) -> Result<bool, VendorDomainError> {
        let vendor = self
            .vendor_repo
            .find_by_id(id)
            .await?
            .ok_or(VendorDomainError::VendorNotFound)?;

        Ok(vendor.owner_id() == owner_id)
    }
}
