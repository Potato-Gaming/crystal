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
pub struct LolLobbyJoinPartyAnalytics {
    #[serde(rename = "eventData", skip_serializing_if = "Option::is_none")]
    pub event_data: Option<serde_json::Value>,
    #[serde(rename = "eventTimestamp", skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<i64>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "numFriendsOnline", skip_serializing_if = "Option::is_none")]
    pub num_friends_online: Option<i32>,
    #[serde(rename = "numOpenFriends", skip_serializing_if = "Option::is_none")]
    pub num_open_friends: Option<i32>,
    #[serde(rename = "numOpenParties", skip_serializing_if = "Option::is_none")]
    pub num_open_parties: Option<i32>,
    #[serde(rename = "numOtherInvites", skip_serializing_if = "Option::is_none")]
    pub num_other_invites: Option<i32>,
    #[serde(rename = "numPartyInvites", skip_serializing_if = "Option::is_none")]
    pub num_party_invites: Option<i32>,
    #[serde(rename = "numTotalInvites", skip_serializing_if = "Option::is_none")]
    pub num_total_invites: Option<i32>,
    #[serde(rename = "partyId", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(rename = "partySize", skip_serializing_if = "Option::is_none")]
    pub party_size: Option<i32>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
}

impl LolLobbyJoinPartyAnalytics {
    pub fn new() -> LolLobbyJoinPartyAnalytics {
        LolLobbyJoinPartyAnalytics {
            event_data: None,
            event_timestamp: None,
            event_type: None,
            game_mode: None,
            num_friends_online: None,
            num_open_friends: None,
            num_open_parties: None,
            num_other_invites: None,
            num_party_invites: None,
            num_total_invites: None,
            party_id: None,
            party_size: None,
            platform_id: None,
        }
    }
}


