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
pub struct LolClashRosterDynamicStateNotification {
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<crate::models::LolClashRosterNotifyReason>,
    #[serde(rename = "rosterDynamicState", skip_serializing_if = "Option::is_none")]
    pub roster_dynamic_state: Option<crate::models::RosterDynamicStateDto>,
    #[serde(rename = "sourcePlayerId", skip_serializing_if = "Option::is_none")]
    pub source_player_id: Option<i64>,
}

impl LolClashRosterDynamicStateNotification {
    pub fn new() -> LolClashRosterDynamicStateNotification {
        LolClashRosterDynamicStateNotification {
            notify_reason: None,
            roster_dynamic_state: None,
            source_player_id: None,
        }
    }
}


