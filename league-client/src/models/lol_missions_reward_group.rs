/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolMissionsRewardGroup {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "localizations", skip_serializing_if = "Option::is_none")]
    pub localizations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "productId", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::LolMissionsSvcReward>>,
}

impl LolMissionsRewardGroup {
    pub fn new() -> LolMissionsRewardGroup {
        LolMissionsRewardGroup {
            active: None,
            id: None,
            internal_name: None,
            localizations: None,
            media: None,
            product_id: None,
            rewards: None,
        }
    }
}


