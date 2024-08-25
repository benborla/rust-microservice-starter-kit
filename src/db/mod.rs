use crate::config::Config;
use crate::error::AppError;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool() -> Result<PgPool, AppError> {
    let config = Config::from_env()?;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .map_err(AppError::DatabaseError)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), AppError> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(AppError::MigrationError)
}
