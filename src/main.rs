/* src/main.rs */

mod bootstrap;
mod router;

#[tokio::main]
async fn main() {
    bootstrap::start().await;
}
