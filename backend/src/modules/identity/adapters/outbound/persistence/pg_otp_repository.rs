use crate::modules::identity::IdentityError;
use crate::modules::identity::adapters::outbound::persistence::otp_row::OtpRow;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpId, OtpPurpose, OtpStatus};
use crate::modules::identity::ports::outbound::OtpRepositoryPort;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgOtpRepository {
    pool: PgPool,
}

impl PgOtpRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

// TODO: Study all about sqlx
#[async_trait]
impl OtpRepositoryPort for PgOtpRepository {
    async fn save(&self, otp: &Otp) -> Result<(), IdentityError> {
        let row = OtpRow::from_entity(otp);

        sqlx::query(
            "INSERT INTO otps \
            (id, identity_id, purpose, code_hash, attempts, expires_at, created_at) \
            VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.identity_id)
        .bind(row.purpose)
        .bind(row.code_hash)
        .bind(row.attempts)
        .bind(row.expires_at)
        .bind(row.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_active(&self, identity_id: &IdentityId, purpose: &OtpPurpose) -> Result<Option<Otp>, IdentityError> {
        let otp_row = sqlx::query_as::<_, OtpRow>(
            "SELECT id, identity_id, purpose, code_hash, status, attempts, consumed_at, expires_at, created_at \
            FROM otps \
            WHERE identity_id = $1 AND purpose = $2 AND status = $3 AND expires_at > NOW()",
        )
        .bind(identity_id.as_uuid())
        .bind(purpose.as_str())
        .bind(OtpStatus::Active.as_str())
        .fetch_optional(&self.pool)
        .await?;

        Ok(otp_row.map(Otp::try_from).transpose()?)
    }

    async fn find_by_id(&self, id: &OtpId) -> Result<Otp, IdentityError> {
        todo!()
    }

    async fn delete(&self, id: &OtpId) -> Result<(), IdentityError> {
        todo!()
    }
}
