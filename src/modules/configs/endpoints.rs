/* src/modules/configs/endpoints.rs */

use crate::{
	common::response,
	modules::configs::{error::ConfigError, value},
};
use axum::{body::Bytes, extract::Path, response::IntoResponse};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use fancy_log::{LogLevel, log};
use serde_json::{Value, json};

// --- Handlers for /v1/config/* ---

/// Helper to convert URL path segments (e.g., "a/b/c") into a dot-separated key ("a.b.c").
fn path_to_key(path: String) -> String {
	path.trim_start_matches('/').replace('/', ".")
}

/// Check if a string is safe for JSON "raw" mode
fn is_json_safe(s: &str) -> bool {
	!s.chars()
		.any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t')
}

/// Handles GET requests to retrieve a value as JSON with type info.
pub async fn get_handler(Path(key): Path<String>) -> Result<impl IntoResponse, ConfigError> {
	let key = path_to_key(key);
	let data = value::get(&key).await?;

	let (value, vtype) = match String::from_utf8(data.clone()) {
		Ok(s) if is_json_safe(&s) => {
			// try json first
			if let Ok(json_val) = serde_json::from_str::<Value>(&s) {
				(json_val, "json")
			} else {
				(json!(s), "string")
			}
		}
		_ => (json!(BASE64_STANDARD.encode(&data)), "base64"),
	};

	let preview = {
		let s = match &value {
			Value::String(s) => s.clone(),
			v => v.to_string(), // array/object/number/bool/null
		};
		// Check the number of characters, not bytes.
		if s.chars().count() <= 100 {
			s
		} else {
			// Safely take the first 100 characters and append "...".
			// This will not panic on multi-byte characters.
			let truncated: String = s.chars().take(90).collect();
			format!("{}...", truncated)
		}
	};

	log(
		LogLevel::Debug,
		&format!("✓ GET Value [{}]: {}", key, preview),
	);

	Ok(response::success(json!({
		"key": key,
		"value": value,
		"type": vtype,
	})))
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
