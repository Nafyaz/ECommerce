use crate::modules::identity::IdentityId;
use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::vendor::domain::value_objects::OwnerId;
use crate::modules::vendor::ports::outbound::{VendorIdentityPort, VendorIdentityPortError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct VendorIdentityQueryAdapter {
    identity_queries: Arc<dyn IdentityQueryPort>,
}

impl VendorIdentityQueryAdapter {
    pub fn new(identity_queries: Arc<dyn IdentityQueryPort>) -> Self {
        Self { identity_queries }
    }
}

#[async_trait]
impl VendorIdentityPort for VendorIdentityQueryAdapter {
    // TODO: It should not use IdentityId directly.
    async fn check_verified(&self, owner_id: &OwnerId) -> Result<bool, VendorIdentityPortError> {
        let identity_id = IdentityId::from_uuid(owner_id.as_uuid().to_owned());

        self.identity_queries
            .check_verified(&identity_id)
            .await
            .map_err(|_| VendorIdentityPortError::Unexpected)
    }
}
