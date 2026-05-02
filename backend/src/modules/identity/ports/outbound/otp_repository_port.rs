use crate::modules::identity::IdentityError;
use crate::modules::identity::domain::entities::Otp;
use crate::modules::identity::domain::value_objects::{IdentityId, OtpId, OtpPurpose};
use async_trait::async_trait;

#[async_trait]
pub trait OtpRepositoryPort: Send + Sync {
    async fn save(&self, otp: &Otp) -> Result<(), IdentityError>;
    async fn update(&self, otp: &Otp) -> Result<(), IdentityError>;
    async fn find_active(&self, identity_id: &IdentityId, purpose: &OtpPurpose) -> Result<Option<Otp>, IdentityError>;
    async fn find_by_id(&self, id: &OtpId) -> Result<Otp, IdentityError>;
    async fn delete(&self, id: &OtpId) -> Result<(), IdentityError>;
}
