use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginUserResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
