use crate::modules::identity::application::IdentityAppError;
use crate::modules::identity::domain::entities::Identity;
use crate::modules::identity::domain::value_objects::{IdentityId, IdentityStatus};
use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::identity::ports::outbound::IdentityRepositoryPort;
use async_trait::async_trait;
use std::sync::Arc;

pub struct IdentityQueryService {
    identity_repo: Arc<dyn IdentityRepositoryPort>,
}

impl IdentityQueryService {
    pub fn new(identity_repo: Arc<dyn IdentityRepositoryPort>) -> Self {
        Self { identity_repo }
    }
}

#[async_trait]
impl IdentityQueryPort for IdentityQueryService {
    async fn get_identity_by_id(&self, identity_id: &IdentityId) -> Result<Identity, IdentityAppError> {
        self.identity_repo
            .find_by_id(identity_id)
            .await?
            .ok_or(IdentityAppError::IdentityNotFound)
    }

    async fn check_verified(&self, identity_id: &IdentityId) -> Result<bool, IdentityAppError> {
        let identity = self
            .identity_repo
            .find_by_id(identity_id)
            .await?
            .ok_or(IdentityAppError::IdentityNotFound)?;

        Ok(*identity.status() == IdentityStatus::Verified)
    }
}
