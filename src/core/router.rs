/* src/core/router.rs */

use crate::{
    common::response,
    core::root::root_handler,
    middleware::cors,
    modules::configs::endpoints::{create_handler, delete_handler, get_handler, update_handler},
};
use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};

/// The main function to create and configure all application routes.
pub fn create_router() -> Router {
    Router::new()
        // The root endpoint providing application metadata.
        .route("/", get(root_handler))
        // Config management endpoints.
        .route("/v1/config/{*key}", get(get_handler))
        .route("/v1/config/{*key}", post(create_handler))
        .route("/v1/config/{*key}", put(update_handler))
        .route("/v1/config/{*key}", delete(delete_handler))
        // Fallback handler for any request that doesn't match a route.
        .fallback(not_found_handler)
        // Apply the CORS layer to all routes.
        .layer(cors::create_cors_layer())
}

/// A handler for unmatched routes, returning a 404 response.
async fn not_found_handler() -> impl IntoResponse {
    response::error(StatusCode::NOT_FOUND, "Resource not found.".to_string())
}
