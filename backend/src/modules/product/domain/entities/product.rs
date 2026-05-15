use crate::modules::product::domain::product_domain_error::ProductDomainError;
use crate::modules::product::domain::value_objects::{ProductId, ProductName, SupplierId};
use crate::modules::shared::Money;
use chrono::{DateTime, Utc};

pub struct Product {
    id: ProductId,
    name: ProductName,
    supplier_id: SupplierId,
    price: Money,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Product {
    pub fn new(name: ProductName, supplier_id: SupplierId, price: Money) -> Result<Self, ProductDomainError> {
        if price.amount_minor() < 0 {
            return Err(ProductDomainError::InvalidProductPrice(price.to_string()));
        }

        let now = Utc::now();
        Ok(Self {
            id: ProductId::new(),
            name,
            supplier_id,
            price,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: ProductId,
        name: ProductName,
        supplier_id: SupplierId,
        price: Money,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, ProductDomainError> {
        if price.amount_minor() < 0 {
            return Err(ProductDomainError::InvalidProductPrice(price.to_string()));
        }

        if updated_at < created_at {
            return Err(ProductDomainError::InvalidTimestamps);
        }

        Ok(Self {
            id,
            name,
            supplier_id,
            price,
            created_at,
            updated_at,
        })
    }

    pub fn id(&self) -> ProductId {
        self.id
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
