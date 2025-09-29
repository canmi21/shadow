/* src/modules/site/metadata.rs */

use crate::modules::configs::error::ConfigError;
use crate::modules::site::requirement::ensure_value_exists;
use serde_json::json;

/// Ensures that default metadata values are present in the database.
pub(super) async fn initialize() -> Result<(), ConfigError> {
	ensure_value_exists("site.title", "Lost in code".as_bytes()).await?;
	ensure_value_exists("site.description", "致虚無，心を守。".as_bytes()).await?;
	ensure_value_exists("site.navbar.title", "そらかなた".as_bytes()).await?;
	ensure_value_exists("site.navbar.description", "致虚无，心を守。".as_bytes()).await?;
	ensure_value_exists("site.domain", "canmi.net".as_bytes()).await?;

	// Handle the JSON array for ICP records.
	let icp_json_value = json!([
			{
					"text": "沪ICP备2025141863号",
					"url": "https://beian.miit.gov.cn/",
					"visible": ["zh-CN"]
			},
			{
					"text": "沪公网安备31011702890896号",
					"url": "https://beian.mps.gov.cn/#/query/webSearch?code=31011702890896",
					"visible": ["zh-CN"]
			},
			{
					"text": "萌ICP备20242133号",
					"url": "https://icp.gov.moe/?keyword=20251350",
					"visible": ["und", "en-US", "en-GB", "zh-HK", "es-ES", "fr-FR", "ja-JP" ]
			}
	])
	.to_string();
	ensure_value_exists("site.icp", icp_json_value.as_bytes()).await?;

	Ok(())
}
