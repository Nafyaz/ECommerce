use crate::modules::identity::domain::entities::Identity;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct IdentityRow {
    pub id: Uuid,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl IdentityRow {
    pub fn from_entity(user: &Identity) -> Self {
        Self {
            id: user.id().as_uuid().to_owned(),
            email: user.email().as_str().to_owned(),
            email_verified_at: user.email_verified_at(),
            password_hash: user.password_hash().as_str().to_owned(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
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
}
