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
pub struct LolGameflowGameflowGameData {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "isCustomGame", skip_serializing_if = "Option::is_none")]
    pub is_custom_game: Option<bool>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "playerChampionSelections", skip_serializing_if = "Option::is_none")]
    pub player_champion_selections: Option<Vec<serde_json::Value>>,
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<crate::models::LolGameflowQueue>,
    #[serde(rename = "spectatorsAllowed", skip_serializing_if = "Option::is_none")]
    pub spectators_allowed: Option<bool>,
    #[serde(rename = "teamOne", skip_serializing_if = "Option::is_none")]
    pub team_one: Option<Vec<serde_json::Value>>,
    #[serde(rename = "teamTwo", skip_serializing_if = "Option::is_none")]
    pub team_two: Option<Vec<serde_json::Value>>,
}

impl LolGameflowGameflowGameData {
    pub fn new() -> LolGameflowGameflowGameData {
        LolGameflowGameflowGameData {
            game_id: None,
            game_name: None,
            is_custom_game: None,
            password: None,
            player_champion_selections: None,
            queue: None,
            spectators_allowed: None,
            team_one: None,
            team_two: None,
        }
    }
}


