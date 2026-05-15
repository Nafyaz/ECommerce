use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub sub: Uuid,
    pub token_type: String,
    pub iat: i64,
    pub exp: i64,
}
