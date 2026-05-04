use crate::modules::vendors::application::command_results::CreateVendorResult;
use crate::modules::vendors::application::commands::CreateVendorCommand;
use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::errors::VendorDomainError;
use crate::modules::vendors::ports::inbound::VendorCommandPort;
use crate::modules::vendors::ports::outbound::{IdentityPort, VendorRepositoryPort};
use async_trait::async_trait;
use std::sync::Arc;

pub struct VendorCommandService {
    identity_service: Arc<dyn IdentityPort>,
    vendor_repo: Arc<dyn VendorRepositoryPort>,
}
impl VendorCommandService {
    pub fn new(identity_service: Arc<dyn IdentityPort>, vendor_repo: Arc<dyn VendorRepositoryPort>) -> Self {
        Self {
            identity_service,
            vendor_repo,
        }
    }
}

#[async_trait]
impl VendorCommandPort for VendorCommandService {
    async fn create_vendor(&self, command: CreateVendorCommand) -> Result<CreateVendorResult, VendorDomainError> {
        let is_verified = self
            .identity_service
            .is_verified(command.owner_id())
            .await
            .map_err(|_| VendorDomainError::IdentityPortError)?;

        if !is_verified {
            return Err(VendorDomainError::OwnerNotVerified(
                command.owner_id().as_uuid().to_owned(),
            ));
        }

        let vendor = Vendor::new(command.owner_id().to_owned(), command.name().to_owned())?;

        self.vendor_repo.save(&vendor).await?;

        Ok(CreateVendorResult {
            id: vendor.id().as_uuid().to_owned(),
        })
    }
}
