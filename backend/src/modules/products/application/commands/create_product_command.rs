use crate::modules::products::domain::value_objects::{ProductName, SupplierId};
use crate::modules::products::errors::ProductDomainError;
use crate::modules::shared::{Currency, Money};
use uuid::Uuid;

pub struct CreateProductCommand {
    name: ProductName,
    supplier_id: SupplierId,
    price: Money,
    current_user_id: Uuid,
}

impl CreateProductCommand {
    pub fn new(
        name: String,
        supplier_id: Uuid,
        price_amount: u64,
        price_currency: String,
        current_user_id: Uuid,
    ) -> Result<Self, ProductDomainError> {
        let name = ProductName::new(name)?;
        let supplier_id = SupplierId::from_uuid(supplier_id);
        let price_currency = Currency::from_str(price_currency)
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid currency: {}", e)))?;
        let price = Money::new(price_amount, price_currency)
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid price: {}", e)))?;

        Ok(Self {
            name,
            supplier_id,
            price,
            current_user_id,
        })
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn supplier_id(&self) -> &SupplierId {
        &self.supplier_id
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn current_user_id(&self) -> Uuid {
        self.current_user_id
    }
}
