/* src/modules/configs/sqlite.rs */

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::sync::OnceCell;

// Use a thread-safe, one-time initializer for the database pool.
static DB_POOL: OnceCell<SqlitePool> = OnceCell::const_new();

// Establishes the database connection and stores it in the global static.
// This should only be called once at startup.
pub async fn connect(db_path: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_path)
        .await?;
    DB_POOL
        .set(pool)
        .expect("Database pool already initialized");
    Ok(())
}

// Returns a reference to the initialized database pool.
// Panics if the pool hasn't been initialized via `connect`.
pub fn get_pool() -> &'static SqlitePool {
    DB_POOL
        .get()
        .expect("Database pool has not been initialized.")
}
