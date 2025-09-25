/* src/core/bootstrap.rs */

use crate::core::router;
use anynet::anynet;
use axum::serve;
use dotenvy::dotenv;
use fancy_log::{LogLevel, log, set_log_level};
use lazy_motd::lazy_motd;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::task;

pub async fn start() {
    dotenv().ok();
    setup_logging();
    print_motd();

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(23333);

    let app = router::create_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Run the potentially blocking anynet! macro in a dedicated thread
    // to avoid blocking the Tokio runtime.
    let port_clone = port;
    if let Err(e) = task::spawn_blocking(move || {
        anynet!(port = port_clone, public = true);
    })
    .await
    {
        log(
            LogLevel::Error,
            &format!("anynet panicked in a blocking task: {}", e),
        );
    }

    let listener = TcpListener::bind(addr).await.unwrap();

    serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // This log is now safe because it runs after the server has fully shut down.
    log(LogLevel::Info, "Server has been shut down gracefully.");
}

fn setup_logging() {
    let level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_level = match level.to_lowercase().as_str() {
        "debug" => LogLevel::Debug,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        _ => LogLevel::Info,
    };
    set_log_level(log_level);
}

fn print_motd() {
    lazy_motd!(
        environment = "None",
        build = "Nightly",
        copyright = &[
            "Copyright (c) 2025 Canmi",
            "Released under the AGPL-3.0 License."
        ]
    );
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log(LogLevel::Info, "Signal received, shutdown now...");
}
