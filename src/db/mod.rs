use crate::config::Config;
use crate::error::AppError;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn connect() -> Result<DatabaseConnection, AppError> {
    let config = Config::from_env()?;

    let mut opt = ConnectOptions::new(&config.database_url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    Database::connect(opt)
        .await
        .map_err(AppError::DatabaseError)
}
