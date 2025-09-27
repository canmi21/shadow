/* src/modules/site/navigation.rs */

use crate::modules::configs::error::ConfigError;
use crate::modules::site::requirement::ensure_value_exists;

/// Ensures that default navigation link values are present in the database.
pub(super) async fn initialize() -> Result<(), ConfigError> {
	ensure_value_exists("site.link.me", "/about".as_bytes()).await?;
	ensure_value_exists("site.link.site", "/about-site".as_bytes()).await?;
	ensure_value_exists(
		"site.link.project",
		"https://github.com/canmi21/sora".as_bytes(),
	)
	.await?;
	ensure_value_exists(
		"site.link.repos",
		"https://github.com/canmi21?tab=repositories".as_bytes(),
	)
	.await?;
	ensure_value_exists(
		"site.link.opensource",
		"https://github.com/canmi21".as_bytes(),
	)
	.await?;
	ensure_value_exists("site.link.sponsor", "https://afdian.com/a/canmi".as_bytes()).await?;
	ensure_value_exists("site.link.git", "https://github.com/canmi21".as_bytes()).await?;
	ensure_value_exists("site.link.email", "t@canmi.icu".as_bytes()).await?;
	ensure_value_exists("site.link.status", "https://status.canmi.icu".as_bytes()).await?;
	Ok(())
}
