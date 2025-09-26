/* src/modules/configs/endpoints.rs */

use crate::{
    common::response,
    modules::configs::{error::ConfigError, value},
};
use axum::{
    body::Bytes,
    extract::Path,
    http::{StatusCode, header},
    response::IntoResponse,
};
use fancy_log::{LogLevel, log};
use serde_json::json;

// --- Handlers for /v1/config/* ---

/// Helper to convert URL path segments (e.g., "a/b/c") into a dot-separated key ("a.b.c").
fn path_to_key(path: String) -> String {
    path.trim_start_matches('/').replace('/', ".")
}

/// Handles GET requests to retrieve a value.
pub async fn get_handler(Path(key): Path<String>) -> Result<impl IntoResponse, ConfigError> {
    let key = path_to_key(key);
    let data = value::get(&key).await?;
    let preview = String::from_utf8_lossy(&data);
    let preview = if preview.len() <= 100 {
        preview.into_owned()
    } else {
        format!("{}...", &preview[..100])
    };
    log(
        LogLevel::Debug,
        &format!("✓ GET Value [{}]: {}", key, preview),
    );
    // Return raw bytes, letting the client decide how to interpret it.
    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/octet-stream")],
        data,
    ))
}

/// Handles POST requests to create a new key-value pair.
pub async fn create_handler(
    Path(key): Path<String>,
    body: Bytes,
) -> Result<impl IntoResponse, ConfigError> {
    let key = path_to_key(key);
    value::create(&key, &body).await?;
    Ok(response::success(
        json!({ "status": "created", "key": key }),
    ))
}

/// Handles PUT requests to update an existing value.
pub async fn update_handler(
    Path(key): Path<String>,
    body: Bytes,
) -> Result<impl IntoResponse, ConfigError> {
    let key = path_to_key(key);
    value::update(&key, &body).await?;
    Ok(response::success(
        json!({ "status": "updated", "key": key }),
    ))
}

/// Handles DELETE requests to remove a key-value pair.
pub async fn delete_handler(Path(key): Path<String>) -> Result<impl IntoResponse, ConfigError> {
    let key = path_to_key(key);
    value::delete(&key).await?;
    Ok(response::success(
        json!({ "status": "deleted", "key": key }),
    ))
}
