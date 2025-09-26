/* src/modules/site/requirement.rs */

use crate::modules::{
	configs::{error::ConfigError, value},
	site::{inception, metadata, owner},
};
use fancy_log::{LogLevel, log};

/// A helper function to create a key-value pair only if it doesn't already exist.
pub async fn ensure_value_exists(key: &str, default_value: &[u8]) -> Result<(), ConfigError> {
	if !value::exists(key).await? {
		log(
			LogLevel::Debug,
			&format!("Initializing missing config key: {}", key),
		);
		value::create(key, default_value).await?;
	}
	Ok(())
}

/// Runs all checks to ensure default site configuration values are present.
/// This is intended to be called once on server startup.
pub async fn ensure_defaults() -> Result<(), ConfigError> {
	metadata::initialize().await?;
	inception::initialize().await?;
	owner::initialize().await?;
	Ok(())
}
