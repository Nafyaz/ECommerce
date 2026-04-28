use crate::modules::users::domain::entities::User;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct UserRow {
    id: Uuid,
    identity_id: Uuid,
    name: String,
    phone: Option<String>,
    phone_verified_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRow {
    pub fn from_entity(user: &User) -> Self {
        Self {
            id: user.id().as_uuid().to_owned(),
            identity_id: user.identity_id().as_uuid().to_owned(),
            name: user.name().as_str().to_owned(),
            phone: user.phone().as_ref().map(|p| p.as_str().to_owned()),
            phone_verified_at: user.phone_verified_at(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }

    pub fn into_entity(self) -> User {
        User::reconstitute(
            self.id,
            self.identity_id,
            self.name,
            self.phone,
            self.phone_verified_at,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn identity_id(&self) -> &Uuid {
        &self.identity_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn phone(&self) -> &Option<String> {
        &self.phone
    }

    pub fn phone_verified_at(&self) -> Option<DateTime<Utc>> {
        self.phone_verified_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
