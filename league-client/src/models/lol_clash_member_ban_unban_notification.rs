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
pub struct LolClashMemberBanUnbanNotification {
    #[serde(rename = "notifyPlayerId", skip_serializing_if = "Option::is_none")]
    pub notify_player_id: Option<i64>,
    #[serde(rename = "notifyPuuid", skip_serializing_if = "Option::is_none")]
    pub notify_puuid: Option<String>,
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<crate::models::LolClashNotifyReason>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "tournaments", skip_serializing_if = "Option::is_none")]
    pub tournaments: Option<Vec<crate::models::MemberBanUnbanTournament>>,
}

impl LolClashMemberBanUnbanNotification {
    pub fn new() -> LolClashMemberBanUnbanNotification {
        LolClashMemberBanUnbanNotification {
            notify_player_id: None,
            notify_puuid: None,
            notify_reason: None,
            player_id: None,
            reason: None,
            tournaments: None,
        }
    }
}


