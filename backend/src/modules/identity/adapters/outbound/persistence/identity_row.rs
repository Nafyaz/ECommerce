use crate::modules::identity::domain::entities::Identity;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct IdentityRow {
    id: Uuid,
    email: String,
    email_verified_at: Option<DateTime<Utc>>,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IdentityRow {
    pub fn from_entity(identity: &Identity) -> Self {
        Self {
            id: identity.id().as_uuid().to_owned(),
            email: identity.email().as_str().to_owned(),
            email_verified_at: identity.email_verified_at(),
            password_hash: identity.password_hash().as_str().to_owned(),
            created_at: identity.created_at(),
            updated_at: identity.updated_at(),
        }
    }

    pub fn into_entity(self) -> Identity {
        Identity::reconstitute(
            self.id,
            self.email,
            self.email_verified_at,
            self.password_hash,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn email_verified_at(&self) -> Option<DateTime<Utc>> {
        self.email_verified_at
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
