/* src/main.rs */

use dotenvy::dotenv;
use fancy_log::{LogLevel, log, set_log_level};
use lazy_motd::lazy_motd;
use std::env;

fn main() {
    dotenv().ok();
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_level = match level.to_lowercase().as_str() {
        "debug" => LogLevel::Debug,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    };
    set_log_level(log_level);
    lazy_motd!(
        environment = "None",
        build = "Nightly",
        copyright = &[
            "Copyright (c) 2025 Canmi",
            "Released under the AGPL-3.0 License."
        ]
    );
    log(LogLevel::Info, "Hi");
}
