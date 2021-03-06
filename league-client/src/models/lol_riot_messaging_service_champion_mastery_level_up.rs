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
pub struct LolRiotMessagingServiceChampionMasteryLevelUp {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "championLevel", skip_serializing_if = "Option::is_none")]
    pub champion_level: Option<i64>,
    #[serde(rename = "hasLeveledUp", skip_serializing_if = "Option::is_none")]
    pub has_leveled_up: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
}

impl LolRiotMessagingServiceChampionMasteryLevelUp {
    pub fn new() -> LolRiotMessagingServiceChampionMasteryLevelUp {
        LolRiotMessagingServiceChampionMasteryLevelUp {
            champion_id: None,
            champion_level: None,
            has_leveled_up: None,
            id: None,
            player_id: None,
        }
    }
}


