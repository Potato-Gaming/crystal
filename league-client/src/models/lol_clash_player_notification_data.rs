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
pub struct LolClashPlayerNotificationData {
    #[serde(rename = "keySuffix", skip_serializing_if = "Option::is_none")]
    pub key_suffix: Option<String>,
    #[serde(rename = "notification", skip_serializing_if = "Option::is_none")]
    pub notification: Option<crate::models::LolClashPlayerNotification>,
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<crate::models::LolClashNotifyReason>,
    #[serde(rename = "rosterNotifyReason", skip_serializing_if = "Option::is_none")]
    pub roster_notify_reason: Option<crate::models::LolClashRosterNotifyReason>,
    #[serde(rename = "sourceSummonerId", skip_serializing_if = "Option::is_none")]
    pub source_summoner_id: Option<i64>,
    #[serde(rename = "targetSummonerId", skip_serializing_if = "Option::is_none")]
    pub target_summoner_id: Option<i64>,
    #[serde(rename = "tournamentNotifyReason", skip_serializing_if = "Option::is_none")]
    pub tournament_notify_reason: Option<crate::models::LolClashTournamentNotifyReason>,
}

impl LolClashPlayerNotificationData {
    pub fn new() -> LolClashPlayerNotificationData {
        LolClashPlayerNotificationData {
            key_suffix: None,
            notification: None,
            notify_reason: None,
            roster_notify_reason: None,
            source_summoner_id: None,
            target_summoner_id: None,
            tournament_notify_reason: None,
        }
    }
}


