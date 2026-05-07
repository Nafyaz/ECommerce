use crate::modules::products::domain::entities::Product;
use crate::modules::products::domain::value_objects::{ProductId, ProductName, SupplierId};
use crate::modules::products::errors::ProductDomainError;
use crate::modules::shared::{Currency, Money};
use chrono::{Date, DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ProductRecord {
    id: Uuid,
    name: String,
    supplier_id: Uuid,
    price_amount: i64,
    price_currency: String,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProductRecord {
    pub fn from_entity(product: &Product) -> Self {
        Self {
            id: product.id().as_uuid().to_owned(),
            name: product.name().as_str().to_owned(),
            supplier_id: product.supplier_id().as_uuid().to_owned(),
            price_amount: product.price().amount().to_owned(),
            price_currency: product.price().currency().as_str().to_owned(),
            created_at: product.created_at(),
            updated_at: product.updated_at(),
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn supplier_id(&self) -> &Uuid {
        &self.supplier_id
    }

    pub fn price_amount(&self) -> &i64 {
        &self.price_amount
    }

    pub fn price_currency(&self) -> &str {
        &self.price_currency
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl TryFrom<ProductRecord> for Product {
    type Error = ProductDomainError;

    fn try_from(product_record: ProductRecord) -> Result<Self, Self::Error> {
        let currency = Currency::from_str(product_record.price_currency)
            .map_err(|e| ProductDomainError::InternalError(e.to_string()))?;

        Product::reconstitute(
            ProductId::from_uuid(product_record.id),
            ProductName::from_str(product_record.name),
            SupplierId::from_uuid(product_record.supplier_id),
            Money::new(product_record.price_amount, currency)
                .map_err(|e| ProductDomainError::InvalidPrice(format!("Invalid price: {}", e)))?,
            product_record.created_at,
            product_record.updated_at,
        )
    }
}
