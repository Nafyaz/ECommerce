use crate::modules::identity::domain::OtpDomainError;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpCodeHash, OtpId, OtpPurpose, OtpStatus};
use chrono::{DateTime, Duration, Utc};

// TODO: Otp expiry duration from config or from purpose?
// TODO: Use totp-rs for only for MFA. Otherwise, just rand + sha2
// TODO: Run crons to clean up expired otps
pub struct Otp {
    id: OtpId,
    identity_id: IdentityId,
    purpose: OtpPurpose,
    code_hash: OtpCodeHash,
    status: OtpStatus,
    attempts: u8,
    consumed_at: Option<DateTime<Utc>>,
    expires_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Otp {
    pub fn new(
        identity_id: IdentityId,
        purpose: OtpPurpose,
        code_hash: OtpCodeHash,
        duration: Duration,
    ) -> Result<Self, OtpDomainError> {
        let now = Utc::now();

        Ok(Self {
            id: OtpId::new(),
            identity_id,
            purpose,
            code_hash,
            status: OtpStatus::Active,
            attempts: 0,
            consumed_at: None,
            expires_at: now + duration,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn reconstitute(
        id: OtpId,
        identity_id: IdentityId,
        purpose: OtpPurpose,
        code_hash: OtpCodeHash,
        status: OtpStatus,
        attempts: u8,
        consumed_at: Option<DateTime<Utc>>,
        expires_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, OtpDomainError> {
        if expires_at < created_at {
            return Err(OtpDomainError::InvalidTimestamps(
                "Otp expires_at cannot be earlier than created_at".to_owned(),
            ));
        }

        Ok(Self {
            id,
            identity_id,
            purpose,
            code_hash,
            status,
            attempts,
            consumed_at,
            expires_at,
            created_at,
            updated_at,
        })
    }

    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }

    pub fn revoke(&mut self) -> Result<(), OtpDomainError> {
        if self.status != OtpStatus::Active {
            return Err(OtpDomainError::InvalidStateTransition);
        }

        let now = Utc::now();

        if self.expires_at < now {
            self.status = OtpStatus::Expired;
            self.updated_at = now;

            return Err(OtpDomainError::InvalidStateTransition);
        }

        self.status = OtpStatus::Revoked;
        self.updated_at = now;

        Ok(())
    }

    pub fn consume(&mut self) -> Result<(), OtpDomainError> {
        if self.status != OtpStatus::Active {
            return Err(OtpDomainError::InvalidStateTransition);
        }

        let now = Utc::now();

        if self.expires_at < now {
            self.status = OtpStatus::Expired;
            self.updated_at = now;

            return Err(OtpDomainError::InvalidStateTransition);
        }

        self.status = OtpStatus::Consumed;
        self.consumed_at = Some(now);
        self.updated_at = now;

        Ok(())
    }

    pub fn id(&self) -> &OtpId {
        &self.id
    }

    pub fn identity_id(&self) -> &IdentityId {
        &self.identity_id
    }

    pub fn purpose(&self) -> &OtpPurpose {
        &self.purpose
    }

    pub fn code_hash(&self) -> &OtpCodeHash {
        &self.code_hash
    }

    pub fn status(&self) -> &OtpStatus {
        &self.status
    }

    pub fn attempts(&self) -> u8 {
        self.attempts
    }

    pub fn consumed_at(&self) -> Option<DateTime<Utc>> {
        self.consumed_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}
