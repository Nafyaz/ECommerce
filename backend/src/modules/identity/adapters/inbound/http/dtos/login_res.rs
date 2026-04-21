use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginUserResponse {
    pub token: String,
}
