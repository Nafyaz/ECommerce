use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserResult {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}
