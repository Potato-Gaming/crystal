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
pub struct Punishment {
    #[serde(rename = "permaBan", skip_serializing_if = "Option::is_none")]
    pub perma_ban: Option<bool>,
    #[serde(rename = "playerFacingMessage", skip_serializing_if = "Option::is_none")]
    pub player_facing_message: Option<String>,
    #[serde(rename = "punishedForChatLogs", skip_serializing_if = "Option::is_none")]
    pub punished_for_chat_logs: Option<Vec<String>>,
    #[serde(rename = "punishedForGameIds", skip_serializing_if = "Option::is_none")]
    pub punished_for_game_ids: Option<Vec<i64>>,
    #[serde(rename = "punishedUntilDateMillis", skip_serializing_if = "Option::is_none")]
    pub punished_until_date_millis: Option<i64>,
    #[serde(rename = "punishmentLengthGames", skip_serializing_if = "Option::is_none")]
    pub punishment_length_games: Option<i64>,
    #[serde(rename = "punishmentLengthMillis", skip_serializing_if = "Option::is_none")]
    pub punishment_length_millis: Option<i64>,
    #[serde(rename = "punishmentReason", skip_serializing_if = "Option::is_none")]
    pub punishment_reason: Option<String>,
    #[serde(rename = "punishmentType", skip_serializing_if = "Option::is_none")]
    pub punishment_type: Option<String>,
}

impl Punishment {
    pub fn new() -> Punishment {
        Punishment {
            perma_ban: None,
            player_facing_message: None,
            punished_for_chat_logs: None,
            punished_for_game_ids: None,
            punished_until_date_millis: None,
            punishment_length_games: None,
            punishment_length_millis: None,
            punishment_reason: None,
            punishment_type: None,
        }
    }
}


