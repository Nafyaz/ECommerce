// use crate::modules::identity::IdentityDomainError;
// use crate::modules::identity::domain::entities::User;
// use crate::modules::identity::domain::value_objects::UserId;
// use crate::modules::identity::ports::inbound::UserQueryPort;
// use crate::modules::identity::ports::outbound::UserRepositoryPort;
// use async_trait::async_trait;
// use std::sync::Arc;
//
// pub struct UserQueryService {
//     user_repo: Arc<dyn UserRepositoryPort>,
// }
//
// impl UserQueryService {
//     pub fn new(user_repo: Arc<dyn UserRepositoryPort>) -> Self {
//         Self { user_repo }
//     }
// }
//
// #[async_trait]
// impl UserQueryPort for UserQueryService {
//     async fn get_user_by_id(&self, user_id: UserId) -> Result<User, IdentityDomainError> {
//         todo!()
//     }
// }
