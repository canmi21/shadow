/* src/main.rs */

pub mod common;
pub mod core;
pub mod modules;

#[tokio::main]
async fn main() {
    core::bootstrap::start().await;
}
