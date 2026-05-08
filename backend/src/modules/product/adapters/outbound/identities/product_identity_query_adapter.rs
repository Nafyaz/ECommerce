use crate::modules::identity::ports::inbound::IdentityQueryPort;
use crate::modules::identity::{IdentityError, IdentityId};
use crate::modules::product::domain::value_objects::ProductActorId;
use crate::modules::product::ports::outbound::{ProductIdentityPort, ProductIdentityPortError};
use async_trait::async_trait;
use std::sync::Arc;

pub struct ProductIdentityQueryAdapter {
    identity_queries: Arc<dyn IdentityQueryPort>,
}

impl ProductIdentityQueryAdapter {
    pub fn new(identity_queries: Arc<dyn IdentityQueryPort>) -> Self {
        Self { identity_queries }
    }
}

#[async_trait]
impl ProductIdentityPort for ProductIdentityQueryAdapter {
    async fn check_verified(&self, actor_id: ProductActorId) -> Result<bool, ProductIdentityPortError> {
        let identity_id = IdentityId::from_uuid(actor_id.as_uuid());

        self.identity_queries
            .check_verified(&identity_id)
            .await
            .map_err(|error| match error {
                IdentityError::IdentityNotFound => ProductIdentityPortError::NotFound,
                IdentityError::InternalError(_) => ProductIdentityPortError::Unavailable,
                _ => ProductIdentityPortError::Unexpected,
            })
    }
}
