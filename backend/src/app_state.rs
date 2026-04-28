use crate::modules::identity::IdentityState;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub identity_state: IdentityState,
    // pub vendor_state: VendorState,
    // pub user_state: UserState,
    // pub product_state: ProductState,
}

impl FromRef<AppState> for IdentityState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.identity_state.clone()
    }
}
