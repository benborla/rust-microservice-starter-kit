use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database Migration Error: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Server Error: {0}")]
    ServerError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MigrationError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Migration Error",
            ),
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error, need more details",
            ),
            AppError::ConfigError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration Error"),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::ServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Server Error"),
        };

        (status, error_message.to_string()).into_response()
    }
}
