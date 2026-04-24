use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct CreateUserResult {
    id: Uuid,
    name: String,
    created_at: DateTime<Utc>,
}

impl CreateUserResult {
    pub fn new(id: Uuid, name: String, created_at: DateTime<Utc>) -> Self {
        Self { id, name, created_at }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
