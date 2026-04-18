use crate::config::database::DatabaseConfig;
use secrecy::ExposeSecret;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections())
        .connect(config.url().expose_secret())
        .await
}
