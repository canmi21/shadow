/* src/router.rs */

use axum::{Router, response::Html, routing::get};

pub fn create_router() -> Router {
    Router::new().route("/", get(hello_world))
}

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
