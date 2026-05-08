use crate::modules::product::domain::value_objects::{ProductActorId, ProductName, SupplierId};
use crate::modules::product::errors::ProductDomainError;
use crate::modules::shared::{Currency, Money};
use std::str::FromStr;
use uuid::Uuid;

pub struct CreateProductCommand {
    current_actor_id: ProductActorId,
    name: ProductName,
    supplier_id: SupplierId,
    price: Money,
}

impl CreateProductCommand {
    pub fn new(
        current_user_id: Uuid,
        name: String,
        supplier_id: Uuid,
        price_amount_minor: i64,
        price_currency: String,
    ) -> Result<Self, ProductDomainError> {
        let current_actor_id = ProductActorId::from_uuid(current_user_id);
        let name = ProductName::new(name)?;
        let supplier_id = SupplierId::from_uuid(supplier_id);
        let price_currency = Currency::from_str(price_currency.as_str())
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid currency: {}", e)))?;
        let price = Money::new(price_amount_minor, price_currency)
            .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid price: {}", e)))?;

        Ok(Self {
            current_actor_id,
            name,
            supplier_id,
            price,
        })
    }

    pub fn name(&self) -> &ProductName {
        &self.name
    }

    pub fn supplier_id(&self) -> SupplierId {
        self.supplier_id
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn current_actor_id(&self) -> ProductActorId {
        self.current_actor_id
    }
}
