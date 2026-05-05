use crate::infrastructure::http::middleware::AuthState;
use crate::modules::identity::IdentityHttpState;
use crate::modules::users::UserHttpState;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub auth_state: AuthState,
    pub identity_http_state: IdentityHttpState,
    pub user_http_state: UserHttpState,
    // pub vendor_http_state: VendorState,
    // pub product_http_state: ProductState,
}

impl FromRef<AppState> for AuthState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.auth_state.clone()
    }
}

impl FromRef<AppState> for IdentityHttpState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.identity_http_state.clone()
    }
}

impl FromRef<AppState> for UserHttpState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.user_http_state.clone()
    }
}
