use crate::modules::products::domain::value_objects::{ProductId, ProductName, VendorId};
use crate::modules::products::errors::ProductDomainError;
use crate::modules::shared::Money;
use chrono::{DateTime, Utc};

pub struct Product {
    id: ProductId,
    name: ProductName,
    vendor_id: VendorId,
    price: Money,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(name: ProductName, vendor_id: VendorId, price: Money) -> Result<Self, ProductDomainError> {
        let now = Utc::now();
        Ok(Self {
            id: ProductId::new(),
            name,
            vendor_id,
            price,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn id(&self) -> &ProductId {
        &self.id
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
