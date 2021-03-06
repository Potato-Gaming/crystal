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
pub struct LolChatRankedQueueStats {
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<crate::models::LolChatLeagueDivision>,
    #[serde(rename = "games", skip_serializing_if = "Option::is_none")]
    pub games: Option<i32>,
    #[serde(rename = "isProvisional", skip_serializing_if = "Option::is_none")]
    pub is_provisional: Option<bool>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<crate::models::LolChatLeagueQueueType>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<crate::models::LolChatLeagueTier>,
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,
}

impl LolChatRankedQueueStats {
    pub fn new() -> LolChatRankedQueueStats {
        LolChatRankedQueueStats {
            division: None,
            games: None,
            is_provisional: None,
            queue_type: None,
            tier: None,
            wins: None,
        }
    }
}


