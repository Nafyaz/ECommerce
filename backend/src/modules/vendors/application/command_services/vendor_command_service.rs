use crate::modules::shared::AppError;
use crate::modules::vendors::application::command_results::CreateVendorResult;
use crate::modules::vendors::application::commands::CreateVendorCommand;
use crate::modules::vendors::domain::entities::Vendor;
use crate::modules::vendors::domain::value_objects::OwnerId;
use crate::modules::vendors::ports::inbound::VendorCommandPort;
use crate::modules::vendors::ports::outbound::VendorRepositoryPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct VendorCommandService {
    vendor_repo: Arc<dyn VendorRepositoryPort>,
}
impl VendorCommandService {
    pub fn new(vendor_repo: Arc<dyn VendorRepositoryPort>) -> Self {
        Self { vendor_repo }
    }
}

#[async_trait]
impl VendorCommandPort for VendorCommandService {
    async fn create_vendor(
        &self,
        command: CreateVendorCommand,
        owner_id: OwnerId,
    ) -> Result<CreateVendorResult, AppError> {
        let vendor = Vendor::new(command.name, owner_id);
        self.vendor_repo.save(&vendor).await?;
        Ok(CreateVendorResult {
            id: vendor.id().as_uuid().to_owned(),
        })
    }
}
