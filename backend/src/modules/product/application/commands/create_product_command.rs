use crate::modules::product::application::ProductAppError;
use crate::modules::product::domain::value_objects::{ProductActorId, ProductName, SupplierId};
use crate::modules::shared::Money;
use uuid::Uuid;

pub struct CreateProductCommand {
    current_actor_id: ProductActorId,
    name: ProductName,
    supplier_id: SupplierId,
    price: Money,
}

impl CreateProductCommand {
    pub fn new(
        current_actor_id: Uuid,
        name: String,
        supplier_id: Uuid,
        price_amount_minor: i64,
        price_currency: String,
    ) -> Result<Self, ProductAppError> {
        let current_actor_id = ProductActorId::from_uuid(current_actor_id);
        let name = ProductName::new(name)?;
        let supplier_id = SupplierId::from_uuid(supplier_id);
        let price = Money::new(price_amount_minor, price_currency)?;

        Ok(Self {
            current_actor_id,
            name,
            supplier_id,
            price,
        })
    }

    pub fn current_actor_id(&self) -> ProductActorId {
        self.current_actor_id
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
}
