/* src/core/router.rs */

use crate::common::response;
use axum::{Router, response::IntoResponse, routing::get};
use serde_json::json;

pub fn create_router() -> Router {
    Router::new().route("/", get(hello_world))
}

/// Returns a standardized JSON success response.
async fn hello_world() -> impl IntoResponse {
    // Use the `json!` macro for simple, one-off JSON objects.
    // This is then passed to our standardized `success` function.
    response::success(json!({
        "message": "Hello, World!"
    }))
}
