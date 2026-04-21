// use async_trait::async_trait;
// use axum::{
//     extract::FromRequestParts,
//     http::{StatusCode, request::Parts},
// };
//
// #[derive(Clone, Debug)]
// pub struct CurrentUser {
//     pub user_id: String,
//     // pub roles: Vec<String>,
// }
//
// #[async_trait]
// impl<S> FromRequestParts<S> for CurrentUser
// where
//     S: Send + Sync,
// {
//     type Rejection = (StatusCode, &'static str);
//
//     async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
//         parts
//             .extensions
//             .get::<CurrentUser>()
//             .cloned()
//             .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))
//     }
// }
