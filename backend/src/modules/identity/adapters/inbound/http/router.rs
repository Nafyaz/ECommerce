use crate::modules::identity::adapters::inbound::http::IdentityHttpState;
use crate::modules::identity::adapters::inbound::http::handlers::{
    login_handler, register_handler, resend_otp_handler, verify_otp_handler,
};
use axum::Router;
use axum::routing::post;

pub fn create_router<S>() -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    IdentityHttpState: axum::extract::FromRef<S>,
{
    // TODO: Add rate limiting
    let public_router = Router::new()
        .route("/register", post(register_handler::handle))
        .route("/resend-otp", post(resend_otp_handler::handle))
        .route("/verify-otp", post(verify_otp_handler::handle))
        .route("/login", post(login_handler::handle));
    // .route("/forgot-password", post(forgot_password_handler::handle));

    Router::new().merge(public_router)
}
