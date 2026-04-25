use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct RegisterResult {
    id: Uuid,
    created_at: DateTime<Utc>,
}

impl RegisterResult {
    pub fn new(id: Uuid, created_at: DateTime<Utc>) -> Self {
        Self { id, created_at }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
