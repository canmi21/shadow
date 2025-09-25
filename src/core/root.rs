/* src/core/root.rs */

use crate::common::response;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::json;
use std::env;

pub async fn root_handler() -> impl IntoResponse {
    // --- Package Info (from Cargo.toml) ---
    let pkg_version = env!("CARGO_PKG_VERSION");
    let repository = env!("CARGO_PKG_REPOSITORY");
    let license = env!("CARGO_PKG_LICENSE");

    // --- Build Info (from build.rs) ---
    let git_commit = env!("GIT_COMMIT_SHORT");
    let rustc_version = env!("RUSTC_FULL_VERSION");
    let cargo_version = env!("CARGO_FULL_VERSION");
    let build_timestamp = env!("BUILD_TIMESTAMP");

    // --- Platform & Dynamic Info (runtime) ---
    let arch = env::consts::ARCH;
    let os = env::consts::OS;
    let request_timestamp = Utc::now().to_rfc3339();

    response::success(json!({
        "package": {
            "author": "Canmi(Canmi21) t@canmi.icu",
            "version": format!("Shadow v{}", pkg_version),
            "license": license,
            "repository": repository,
        },
        "build": {
            "commit": git_commit,
            "rust": rustc_version,
            "cargo": cargo_version,
            "timestamp": build_timestamp,
        },
        "runtime": {
            "arch": arch,
            "platform": os,
        },
        "timestamp": request_timestamp,
    }))
}
