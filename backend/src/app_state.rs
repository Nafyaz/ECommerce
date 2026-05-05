use crate::modules::identity::IdentityHttpState;
use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub identity_http_state: IdentityHttpState,
    // pub vendor_http_state: VendorState,
    // pub user_http_state: UserState,
    // pub product_http_state: ProductState,
}

impl FromRef<AppState> for IdentityHttpState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.identity_http_state.clone()
    }
}
