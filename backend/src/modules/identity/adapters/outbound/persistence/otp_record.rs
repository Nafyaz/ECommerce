use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpCodeHash, OtpId, OtpPurpose, OtpStatus};
use crate::modules::identity::ports::outbound::OtpRepositoryError;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct OtpRecord {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub purpose: String,
    pub code_hash: String,
    pub status: String,
    pub attempts: i16,
    pub consumed_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OtpRecord {
    pub fn from_entity(otp: &Otp) -> Self {
        Self {
            id: otp.id().as_uuid(),
            identity_id: otp.identity_id().as_uuid(),
            purpose: otp.purpose().as_str().to_owned(),
            code_hash: otp.code_hash().as_str().to_owned(),
            status: otp.status().as_str().to_owned(),
            attempts: otp.attempts() as i16,
            consumed_at: otp.consumed_at(),
            expires_at: otp.expires_at(),
            created_at: otp.created_at(),
            updated_at: otp.updated_at(),
        }
    }
}

impl TryFrom<OtpRecord> for Otp {
    type Error = OtpRepositoryError;

    fn try_from(otp_record: OtpRecord) -> Result<Self, Self::Error> {
        Otp::reconstitute(
            OtpId::from_uuid(otp_record.id),
            IdentityId::from_uuid(otp_record.identity_id),
            OtpPurpose::from_str(otp_record.purpose).map_err(|e| OtpRepositoryError::CorruptData(e.to_string()))?,
            OtpCodeHash::from_str(otp_record.code_hash),
            OtpStatus::from_str(otp_record.status).map_err(|e| OtpRepositoryError::CorruptData(e.to_string()))?,
            otp_record.attempts as u8,
            otp_record.consumed_at,
            otp_record.expires_at,
            otp_record.created_at,
            otp_record.updated_at,
        )
        .map_err(|e| OtpRepositoryError::CorruptData(e.to_string()))
    }
}
