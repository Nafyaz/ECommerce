use crate::modules::products::domain::value_objects::ProductName;
use crate::modules::products::errors::ProductDomainError;
use crate::modules::shared::{Currency, Money};
use crate::modules::vendors::VendorId;
use uuid::Uuid;

pub struct CreateProductCommand {
    name: ProductName,
    vendor_id: VendorId,
    price: Money,
    current_user_id: Uuid,
}

impl CreateProductCommand {
    pub fn new(
        name: String,
        vendor_id: Uuid,
        price_amount: u64,
        price_currency: String,
        current_user_id: Uuid,
    ) -> Result<Self, ProductDomainError> {
        let name = ProductName::new(name)?;
        let vendor_id = VendorId::from_uuid(vendor_id);
        let price_currency = Currency::new(price_currency)
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid currency: {}", e)))?;
        let price = Money::new(price_amount, price_currency)
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid price: {}", e)))?;

        Ok(Self {
            name,
            vendor_id,
            price,
            current_user_id,
        })
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn vendor_id(&self) -> &VendorId {
        &self.vendor_id
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn current_user_id(&self) -> Uuid {
        self.current_user_id
    }
}
