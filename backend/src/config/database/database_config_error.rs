use super::read_replica::read_replica_error::ReadReplicaError;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseConfigError {
    #[error(
        "max_connection ({max_connections}) must be greater than or equal to min_connections ({min_connections})"
    )]
    InvalidConnectionLimits {
        max_connections: u32,
        min_connections: u32,
    },

    #[error("connection_timeout must not be zero, got {0:?}")]
    InvalidTimeout(Duration),

    #[error("Failed to configure read replica")]
    InvalidReadReplica(#[from] ReadReplicaError),
}
