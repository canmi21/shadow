/* src/modules/site/inception.rs */

use crate::modules::configs::error::ConfigError;
use crate::modules::site::requirement::ensure_value_exists;

/// Ensures that the site inception year is present in the database.
pub(super) async fn initialize() -> Result<(), ConfigError> {
	ensure_value_exists("site.inception", "2021".as_bytes()).await?;
	Ok(())
}
