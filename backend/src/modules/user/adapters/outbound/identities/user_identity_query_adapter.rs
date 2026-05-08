use crate::modules::identity::IdentityId;
use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::user::domain::value_objects::AccountId;
use crate::modules::user::ports::outbound::{UserIdentityPort, UserIdentityPortError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct UserIdentityQueryAdapter {
    identity_queries: Arc<dyn IdentityQueryPort>,
}

impl UserIdentityQueryAdapter {
    pub fn new(identity_queries: Arc<dyn IdentityQueryPort>) -> Self {
        Self { identity_queries }
    }
}

#[async_trait]
impl UserIdentityPort for UserIdentityQueryAdapter {
    async fn check_verified(&self, account_id: &AccountId) -> Result<bool, UserIdentityPortError> {
        let identity_id = IdentityId::from_uuid(account_id.as_uuid().to_owned());

        self.identity_queries
            .check_verified(&identity_id)
            .await
            .map_err(|_| UserIdentityPortError::Unexpected)
    }
}
