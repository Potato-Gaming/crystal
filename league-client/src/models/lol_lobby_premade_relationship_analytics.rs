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
pub struct LolLobbyPremadeRelationshipAnalytics {
    #[serde(rename = "eventData", skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,
    #[serde(rename = "eventTimestamp", skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<i64>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "friendPlayers", skip_serializing_if = "Option::is_none")]
    pub friend_players: Option<Vec<String>>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "premadePlayers", skip_serializing_if = "Option::is_none")]
    pub premade_players: Option<Vec<String>>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolLobbyPremadeRelationshipAnalytics {
    pub fn new() -> LolLobbyPremadeRelationshipAnalytics {
        LolLobbyPremadeRelationshipAnalytics {
            event_data: None,
            event_timestamp: None,
            event_type: None,
            friend_players: None,
            platform_id: None,
            premade_players: None,
            puuid: None,
            summoner_id: None,
        }
    }
}


