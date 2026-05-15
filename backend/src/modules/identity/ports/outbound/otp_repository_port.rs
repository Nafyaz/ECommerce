use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpId, OtpPurpose};
use crate::modules::identity::ports::outbound::OtpRepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait OtpRepositoryPort: Send + Sync {
    async fn save(&self, otp: &Otp) -> Result<(), OtpRepositoryError>;
    async fn update(&self, otp: &Otp) -> Result<(), OtpRepositoryError>;
    async fn find_active(
        &self,
        identity_id: &IdentityId,
        purpose: &OtpPurpose,
    ) -> Result<Option<Otp>, OtpRepositoryError>;
    async fn find_by_id(&self, id: &OtpId) -> Result<Option<Otp>, OtpRepositoryError>;
    async fn delete(&self, id: &OtpId) -> Result<(), OtpRepositoryError>;
}
