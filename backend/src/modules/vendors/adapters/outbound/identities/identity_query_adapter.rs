use crate::modules::identity::IdentityId;
use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::vendors::OwnerId;
use crate::modules::vendors::ports::outbound::{IdentityPort, IdentityPortError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct IdentityQueryAdapter {
    identity_queries: Arc<dyn IdentityQueryPort>,
}

impl IdentityQueryAdapter {
    pub fn new(identity_queries: Arc<dyn IdentityQueryPort>) -> Self {
        Self { identity_queries }
    }
}

#[async_trait]
impl IdentityPort for IdentityQueryAdapter {
    // TODO: It should not use IdentityId directly.
    async fn check_verified(&self, owner_id: &OwnerId) -> Result<bool, IdentityPortError> {
        let identity_id = IdentityId::from_uuid(owner_id.as_uuid().to_owned());

        self.identity_queries
            .check_verified(&identity_id)
            .await
            .map_err(|_| IdentityPortError::Unexpected)
    }
}
