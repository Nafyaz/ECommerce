use crate::modules::vendors::OwnerId;
use crate::modules::vendors::domain::value_objects::VendorName;
use crate::modules::vendors::errors::VendorDomainError;
use uuid::Uuid;

pub struct CreateVendorCommand {
    owner_id: OwnerId,
    name: VendorName,
}

impl CreateVendorCommand {
    pub fn new(owner_id: Uuid, name: String) -> Result<Self, VendorDomainError> {
        let owner_id = OwnerId::from_uuid(owner_id);
        let name = VendorName::new(name)?;
        Ok(Self { owner_id, name })
    }

    pub fn owner_id(&self) -> &OwnerId {
        &self.owner_id
    }

    pub fn name(&self) -> &VendorName {
        &self.name
    }
}
