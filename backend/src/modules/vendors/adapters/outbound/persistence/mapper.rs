use crate::modules::vendors::domain::entities::Vendor;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct VendorRow {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl VendorRow {
    pub fn from_entity(vendor: &Vendor) -> Self {
        Self {
            id: vendor.id().as_uuid().to_owned(),
            name: vendor.name().as_str().to_owned(),
            owner_id: vendor.owner_id().as_uuid().to_owned(),
            created_at: vendor.created_at(),
            updated_at: vendor.updated_at(),
        }
    }

    pub fn into_entity(self) -> Vendor {
        Vendor::reconstitute(id: self.id)
    }
}
