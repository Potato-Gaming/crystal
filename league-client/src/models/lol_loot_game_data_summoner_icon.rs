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
pub struct LolLootGameDataSummonerIcon {
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl LolLootGameDataSummonerIcon {
    pub fn new() -> LolLootGameDataSummonerIcon {
        LolLootGameDataSummonerIcon {
            icon_path: None,
            id: None,
        }
    }
}


