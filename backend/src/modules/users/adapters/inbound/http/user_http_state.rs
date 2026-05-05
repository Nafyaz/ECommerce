use crate::modules::users::ports::inbound::{UserCommandPort, UserQueryPort};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserHttpState {
    pub command_service: Arc<dyn UserCommandPort>,
    pub query_service: Arc<dyn UserQueryPort>,
}

impl UserHttpState {
    pub fn new(command_service: Arc<dyn UserCommandPort>, query_service: Arc<dyn UserQueryPort>) -> Self {
        Self {
            command_service,
            query_service,
        }
    }
}
