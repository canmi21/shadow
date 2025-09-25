/* src/core/router.rs */

use crate::{
    core::root::root_handler,
    modules::configs::endpoints::{create_handler, delete_handler, get_handler, update_handler},
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};

/// The main function to create and configure all application routes.
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/v1/config/{*key}", get(get_handler))
        .route("/v1/config/{*key}", post(create_handler))
        .route("/v1/config/{*key}", put(update_handler))
        .route("/v1/config/{*key}", delete(delete_handler))
}
