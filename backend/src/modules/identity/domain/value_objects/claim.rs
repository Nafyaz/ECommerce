use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub sub: Uuid,
    pub purpose: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
}
