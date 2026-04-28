use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, PasswordHash};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Identity {
    id: IdentityId,
    email: Email,
    email_verified_at: Option<DateTime<Utc>>,
    password_hash: PasswordHash,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Identity {
    pub fn new(email: Email, password_hash: PasswordHash) -> Result<Self, IdentityError> {
        let now = Utc::now();

        Ok(Self {
            id: IdentityId::new(),
            email,
            email_verified_at: None,
            password_hash,
            created_at: now,
            updated_at: now,
        })
    }

    // TODO: Should it be TryFrom / FromStr + Result?
    pub fn reconstitute(
        id: Uuid,
        email: String,
        email_verified_at: Option<DateTime<Utc>>,
        password_hash: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: IdentityId::from_uuid(id),
            email: Email::from_str(email),
            email_verified_at,
            password_hash: PasswordHash::from_str(password_hash),
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> &IdentityId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn email_verified_at(&self) -> Option<DateTime<Utc>> {
        self.email_verified_at
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
