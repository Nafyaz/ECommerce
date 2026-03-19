use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum AuthConfigError {
    #[error("access_token_ttl must be greater than zero, got {0:?}")]
    ZeroAccessTtl(Duration),

    #[error(
        "refresh_token_ttl ({refresh_token_ttl:?}) must be greater than or equal to access_token_ttl ({access_token_ttl:?})"
    )]
    InvalidTtlOrder {
        access_token_ttl: Duration,
        refresh_token_ttl: Duration,
    },

    #[error("jwt token must be at least 32 characters, got {0}")]
    WeakJwtSecret(usize),
}
