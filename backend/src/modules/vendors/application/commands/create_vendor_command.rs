use crate::modules::vendors::domain::value_objects::VendorName;
use crate::modules::vendors::errors::VendorDomainError;

pub struct CreateVendorCommand {
    name: VendorName,
}

impl CreateVendorCommand {
    pub fn new(name: String) -> Result<Self, VendorDomainError> {
        let name = VendorName::new(name)?;
        Ok(Self { name })
    }
}
