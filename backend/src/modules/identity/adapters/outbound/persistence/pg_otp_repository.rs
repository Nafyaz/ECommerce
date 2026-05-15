use crate::modules::identity::adapters::outbound::persistence::otp_record::OtpRecord;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpId, OtpPurpose, OtpStatus};
use crate::modules::identity::ports::outbound::{OtpRepositoryError, OtpRepositoryPort};
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

fn map_sqlx_error(err: sqlx::Error) -> OtpRepositoryError {
    if let sqlx::Error::Database(database_error) = &err {
        return match database_error.code().as_deref() {
            Some("23505") | Some("23503") => OtpRepositoryError::Conflict,
            _ => {
                tracing::error!("Identity repository database error: {:?}", err);
                OtpRepositoryError::Unexpected
            }
        };
    }

    match err {
        sqlx::Error::PoolClosed | sqlx::Error::PoolTimedOut => OtpRepositoryError::Unavailable,
        sqlx::Error::RowNotFound => OtpRepositoryError::NotFound,
        _ => {
            tracing::error!("Product repository database error: {:?}", err);
            OtpRepositoryError::Unexpected
        }
    }
}

// TODO: Study all about sqlx
#[async_trait]
impl OtpRepositoryPort for PgOtpRepository {
    async fn save(&self, otp: &Otp) -> Result<(), OtpRepositoryError> {
        let row = OtpRecord::from_entity(otp);

        sqlx::query(
            "INSERT INTO otps \
            (id, identity_id, purpose, code_hash, status, attempts, consumed_at, expires_at, created_at) \
            VALUES ($1, $2, $3::otp_purpose, $4, $5::otp_status, $6, $7, $8, $9)",
        )
        .bind(row.id)
        .bind(row.identity_id)
        .bind(row.purpose)
        .bind(row.code_hash)
        .bind(row.status)
        .bind(row.attempts)
        .bind(row.consumed_at)
        .bind(row.expires_at)
        .bind(row.created_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    async fn update(&self, otp: &Otp) -> Result<(), OtpRepositoryError> {
        let row = OtpRecord::from_entity(otp);

        let result = sqlx::query(
            "UPDATE otps \
            SET status = $2::otp_status, attempts = $3, consumed_at = $4, expires_at = $5 \
            WHERE id = $1",
        )
        .bind(row.id)
        .bind(row.status)
        .bind(row.attempts)
        .bind(row.consumed_at)
        .bind(row.expires_at)
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        if result.rows_affected() == 0 {
            return Err(OtpRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn find_active(
        &self,
        identity_id: &IdentityId,
        purpose: &OtpPurpose,
    ) -> Result<Option<Otp>, OtpRepositoryError> {
        let otp_row = sqlx::query_as::<_, OtpRecord>(
            "SELECT id, identity_id, purpose::TEXT, code_hash, status::TEXT, attempts, consumed_at, expires_at, created_at \
            FROM otps \
            WHERE identity_id = $1 AND purpose = $2::otp_purpose AND status = $3::otp_status AND expires_at > NOW()",
        )
        .bind(identity_id.as_uuid())
        .bind(purpose.as_str())
        .bind(OtpStatus::Active.as_str())
        .fetch_optional(&self.pool)
        .await.map_err(map_sqlx_error)?;

        Ok(otp_row.map(Otp::try_from).transpose()?)
    }

    async fn find_by_id(&self, id: &OtpId) -> Result<Option<Otp>, OtpRepositoryError> {
        let record = sqlx::query_as::<_, OtpRecord>(
            "SELECT id, identity_id, purpose::TEXT, code_hash, status::TEXT, attempts, consumed_at, expires_at, consumed_at, created_at \
            FROM otps \
            WHERE id = $1",
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await.map_err(map_sqlx_error)?;

        Ok(record.map(Otp::try_from).transpose()?)
    }

    async fn delete(&self, id: &OtpId) -> Result<(), OtpRepositoryError> {
        todo!()
    }
}
