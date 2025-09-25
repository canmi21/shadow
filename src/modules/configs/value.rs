/* src/modules/configs/value.rs */

use crate::modules::configs::{error::ConfigError, sqlite::get_pool};

// Check if a key exists in the store.
pub async fn exists(key: &str) -> Result<bool, ConfigError> {
    let result = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM kv_store WHERE key = ?)")
        .bind(key)
        .fetch_one(get_pool())
        .await?;
    Ok(result)
}

// Create a new key with a given value.
// Fails if the key already exists.
pub async fn create(key: &str, value: &[u8]) -> Result<(), ConfigError> {
    if exists(key).await? {
        return Err(ConfigError::KeyAlreadyExists(key.to_string()));
    }

    sqlx::query("INSERT INTO kv_store (key, value) VALUES (?, ?)")
        .bind(key)
        .bind(value)
        .execute(get_pool())
        .await?;

    Ok(())
}

// Retrieve the value for a given key.
pub async fn get(key: &str) -> Result<Vec<u8>, ConfigError> {
    sqlx::query_scalar("SELECT value FROM kv_store WHERE key = ?")
        .bind(key)
        .fetch_optional(get_pool())
        .await?
        .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))
}

// Update the value for an existing key.
// Fails if the key does not exist.
pub async fn update(key: &str, value: &[u8]) -> Result<(), ConfigError> {
    if !exists(key).await? {
        return Err(ConfigError::KeyNotFound(key.to_string()));
    }

    sqlx::query("UPDATE kv_store SET value = ? WHERE key = ?")
        .bind(value)
        .bind(key)
        .execute(get_pool())
        .await?;

    Ok(())
}

// Delete a key from the store.
// Immediately vacuums the database to reclaim space.
pub async fn delete(key: &str) -> Result<(), ConfigError> {
    if !exists(key).await? {
        return Err(ConfigError::KeyNotFound(key.to_string()));
    }

    // Step 1: Execute the DELETE statement directly on the pool.
    // This is now allowed because it's not inside an explicit transaction.
    sqlx::query("DELETE FROM kv_store WHERE key = ?")
        .bind(key)
        .execute(get_pool())
        .await?;

    // Step 2: After the delete is successful, execute VACUUM as a separate command.
    sqlx::query("VACUUM").execute(get_pool()).await?;

    Ok(())
}
