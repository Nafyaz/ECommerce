use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, IdentityStatus, PasswordHash};
use crate::modules::identity::ports::outbound::IdentityRepositoryError;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct IdentityRecord {
    id: Uuid,
    email: String,
    password_hash: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl IdentityRecord {
    pub fn from_entity(identity: &Identity) -> Self {
        Self {
            id: identity.id().as_uuid(),
            email: identity.email().as_str().to_owned(),
            password_hash: identity.password_hash().as_str().to_owned(),
            status: identity.status().as_str().to_owned(),
            created_at: identity.created_at(),
            updated_at: identity.updated_at(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl TryFrom<IdentityRecord> for Identity {
    type Error = IdentityRepositoryError;

    fn try_from(identity_record: IdentityRecord) -> Result<Self, Self::Error> {
        Identity::reconstitute(
            IdentityId::from_uuid(identity_record.id),
            Email::new(identity_record.email).map_err(|e| IdentityRepositoryError::CorruptData(e.to_string()))?,
            PasswordHash::from_str(identity_record.password_hash),
            IdentityStatus::from_str(identity_record.status)
                .map_err(|e| IdentityRepositoryError::CorruptData(e.to_string()))?,
            identity_record.created_at,
            identity_record.updated_at,
        )
        .map_err(|e| IdentityRepositoryError::CorruptData(e.to_string()))
    }
}
