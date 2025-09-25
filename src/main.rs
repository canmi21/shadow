/* src/main.rs */

pub mod common;
pub mod core;

#[tokio::main]
async fn main() {
    core::bootstrap::start().await;
}
