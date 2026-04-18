use crate::modules::vendors::domain::value_objects::{OwnerId, VendorId};
use chrono::{DateTime, Utc};

// TODO: Use name VO for validations
pub struct Vendor {
    id: VendorId,
    name: String,
    owner_id: OwnerId,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
