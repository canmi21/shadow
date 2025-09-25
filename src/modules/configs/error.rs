/* src/modules/configs/error.rs */

use crate::common::response;
use axum::{http::StatusCode, response::IntoResponse};
use fancy_log::{LogLevel, log};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Key already exists: {0}")]
    KeyAlreadyExists(String),
}

// Convert our custom errors into HTTP responses.
impl IntoResponse for ConfigError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ConfigError::DbError(e) => {
                log(LogLevel::Error, &format!("Database error: {}", e));
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred.".to_string(),
                )
            }
            ConfigError::IoError(e) => {
                log(LogLevel::Error, &format!("I/O error: {}", e));
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An I/O error occurred.".to_string(),
                )
            }
            ConfigError::KeyNotFound(key) => {
                log(
                    LogLevel::Warn,
                    &format!("Attempted to access non-existent key: {}", key),
                );
                (StatusCode::NOT_FOUND, format!("Key not found: {}", key))
            }
            ConfigError::KeyAlreadyExists(key) => (
                StatusCode::CONFLICT, // 409 Conflict is appropriate here
                format!("Key already exists: {}", key),
            ),
        };

        response::error(status, message).into_response()
    }
}
