use crate::modules::users::domain::entities::User;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CreateUserResponse {
    pub fn from_entity(user: &User) -> Self {
        Self {
            id: user.id().as_uuid().to_owned(),
            name: user.name().to_owned(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
