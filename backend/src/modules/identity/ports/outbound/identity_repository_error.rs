use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityRepositoryError {
    #[error("Identity persistence unavailable")]
    Unavailable,

    #[error("Identity not found")]
    NotFound,

    #[error("Identity persistence conflict")]
    Conflict,

    #[error("Corrupt identity data: {0}")]
    CorruptData(String),

    #[error("Unexpected persistence error")]
    Unexpected,
}
