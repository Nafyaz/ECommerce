use crate::modules::users::domain::value_objects::phone::Phone;
use crate::modules::users::domain::value_objects::user_name::UserName;
use crate::modules::users::domain::value_objects::{IdentityId, UserId};
use chrono::{DateTime, Utc};

//TODO: Manually implement slugs and use them
pub struct User {
    id: UserId,
    identity_id: IdentityId,
    name: UserName,
    phone: Phone,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
