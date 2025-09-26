/* src/modules/site/owner.rs */

use crate::modules::configs::error::ConfigError;
use crate::modules::site::requirement::ensure_value_exists;

/// Ensures that default owner values are present in the database.
pub(super) async fn initialize() -> Result<(), ConfigError> {
	ensure_value_exists("owner.name", "Canmi".as_bytes()).await?;
	ensure_value_exists(
		"owner.quote",
		"Time will take away everything that belongs to you.".as_bytes(),
	)
	.await?;
	ensure_value_exists(
		"owner.bio",
		"Crafting the future in hardware & code.".as_bytes(),
	)
	.await?;

	Ok(())
}
