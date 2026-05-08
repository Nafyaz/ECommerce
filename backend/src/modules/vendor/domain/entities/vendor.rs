use crate::modules::vendor::domain::value_objects::{OwnerId, VendorId, VendorName};
use crate::modules::vendor::errors::VendorDomainError;
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Vendor {
    id: VendorId,
    owner_id: OwnerId,
    name: VendorName,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Vendor {
    pub fn new(owner_id: OwnerId, name: VendorName) -> Result<Self, VendorDomainError> {
        let now = Utc::now();
        Ok(Self {
            id: VendorId::new(),
            owner_id,
            name,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: Uuid,
        name: String,
        owner_id: Uuid,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: VendorId::from_uuid(id),
            name: VendorName::from_str(name),
            owner_id: OwnerId::from_uuid(owner_id),
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &VendorId {
        &self.id
    }

    pub fn name(&self) -> &VendorName {
        &self.name
    }

    pub fn owner_id(&self) -> &OwnerId {
        &self.owner_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
