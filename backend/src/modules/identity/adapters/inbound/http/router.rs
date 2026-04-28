use crate::modules::identity::adapters::inbound::http::IdentityState;
use crate::modules::identity::adapters::inbound::http::handlers::{login_handler, register_handler};
use axum::Router;
use axum::routing::post;

pub fn create_router<S>() -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    IdentityState: axum::extract::FromRef<S>,
{
    let public_router = Router::new()
        .route("/register", post(register_handler::handle))
        .route("/login", post(login_handler::handle));

    Router::new().merge(public_router)
}
