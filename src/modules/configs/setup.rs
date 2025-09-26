/* src/modules/configs/setup.rs */

use crate::modules::configs::{error::ConfigError, sqlite};
use std::{env, fs, path::Path};

// Main initialization function to be called at startup.
pub async fn initialize_database() -> Result<(), ConfigError> {
	let db_path_str =
		env::var("DATABASE_PATH").unwrap_or_else(|_| "/opt/shadow/config.sqlite".to_string());

	let db_path = Path::new(&db_path_str);

	// If the database file doesn't exist, create it and its parent directories.
	if !db_path.exists() {
		if let Some(parent) = db_path.parent() {
			fs::create_dir_all(parent)?;
		}
		fs::File::create(db_path)?;
	}

	// Connect to the database and get a pool.
	let db_uri = format!("sqlite://{}", db_path_str);
	sqlite::connect(&db_uri).await?;
	let pool = sqlite::get_pool();

	// Create the key-value table if it doesn't already exist.
	sqlx::query(
		r#"
        CREATE TABLE IF NOT EXISTS kv_store (
            key TEXT PRIMARY KEY NOT NULL,
            value BLOB NOT NULL
        );
        "#,
	)
	.execute(pool)
	.await?;

	Ok(())
}
