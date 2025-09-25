/* src/core/router.rs */

use crate::core::root::root_handler;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new().route("/", get(root_handler))
}
