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
pub struct LolLobbyQueueReward {
    #[serde(rename = "isChampionPointsEnabled", skip_serializing_if = "Option::is_none")]
    pub is_champion_points_enabled: Option<bool>,
    #[serde(rename = "isIpEnabled", skip_serializing_if = "Option::is_none")]
    pub is_ip_enabled: Option<bool>,
    #[serde(rename = "isXpEnabled", skip_serializing_if = "Option::is_none")]
    pub is_xp_enabled: Option<bool>,
    #[serde(rename = "partySizeIpRewards", skip_serializing_if = "Option::is_none")]
    pub party_size_ip_rewards: Option<Vec<i32>>,
}

impl LolLobbyQueueReward {
    pub fn new() -> LolLobbyQueueReward {
        LolLobbyQueueReward {
            is_champion_points_enabled: None,
            is_ip_enabled: None,
            is_xp_enabled: None,
            party_size_ip_rewards: None,
        }
    }
}


