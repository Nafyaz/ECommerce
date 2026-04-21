use crate::modules::identity::domain::value_objects::UserId;
use chrono::{DateTime, Utc};

pub struct CreateUserResult {
    pub id: UserId,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
