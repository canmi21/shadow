/* src/modules/site/metadata.rs */

use crate::modules::configs::error::ConfigError;
use crate::modules::site::requirement::ensure_value_exists;
use serde_json::json;

/// Ensures that default metadata values are present in the database.
pub(super) async fn initialize() -> Result<(), ConfigError> {
	ensure_value_exists("site.title", "貓窝".as_bytes()).await?;
	ensure_value_exists("site.description", "致虚无，心を守。".as_bytes()).await?;
	ensure_value_exists("site.navbar.title", "觉授の貓窝".as_bytes()).await?;
	ensure_value_exists("site.domain", "canmi.net".as_bytes()).await?;

	// Handle the JSON array for ICP records.
	let icp_json_value = json!([
			{
					"text": "沪ICP备2025141863号",
					"url": "https://beian.miit.gov.cn/"
			},
			{
					"text": "萌ICP备202421033号",
					"url": "https://icp.gov.moe/"
			}
	])
	.to_string();
	ensure_value_exists("site.icp", icp_json_value.as_bytes()).await?;

	Ok(())
}
