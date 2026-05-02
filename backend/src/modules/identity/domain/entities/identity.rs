use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::value_objects::{Email, IdentityId, IdentityStatus, PasswordHash};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// TODO: should I add email_verified_at?
pub struct Identity {
    id: IdentityId,
    email: Email,
    password_hash: PasswordHash,
    status: IdentityStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Identity {
    pub fn new(email: Email, password_hash: PasswordHash) -> Result<Self, IdentityError> {
        let now = Utc::now();

        Ok(Self {
            id: IdentityId::new(),
            email,
            password_hash,
            status: IdentityStatus::Pending,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: IdentityId,
        email: Email,
        password_hash: PasswordHash,
        status: IdentityStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, IdentityError> {
        if updated_at < created_at {
            return Err(IdentityError::InternalError(
                "identity updated_at cannot be earlier than created_at".to_owned(),
            ));
        }

        Ok(Self {
            id,
            email,
            password_hash,
            status,
            created_at,
            updated_at,
        })
    }

    pub fn verify_identity(&mut self) {
        let now = Utc::now();
        self.status = IdentityStatus::Verified;
        self.updated_at = now;
    }

    pub fn id(&self) -> &IdentityId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn status(&self) -> &IdentityStatus {
        &self.status
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
