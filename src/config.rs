use crate::error::AppError;
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        // Load .env file
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::ConfigError("DATABASE_URL has not been set".to_string()))?;

        Ok(Config { database_url })
    }
}
