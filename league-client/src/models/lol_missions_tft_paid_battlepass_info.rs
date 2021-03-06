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
pub struct LolMissionsTftPaidBattlepassInfo {
    #[serde(rename = "backgroundImageUrl", skip_serializing_if = "Option::is_none")]
    pub background_image_url: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    #[serde(rename = "premium", skip_serializing_if = "Option::is_none")]
    pub premium: Option<bool>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LolMissionsTftPaidBattlepassInfo {
    pub fn new() -> LolMissionsTftPaidBattlepassInfo {
        LolMissionsTftPaidBattlepassInfo {
            background_image_url: None,
            description: None,
            end_date: None,
            premium: None,
            start_date: None,
            title: None,
        }
    }
}


