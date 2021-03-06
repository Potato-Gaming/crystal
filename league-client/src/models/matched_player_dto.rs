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
pub struct MatchedPlayerDto {
    #[serde(rename = "captain", skip_serializing_if = "Option::is_none")]
    pub captain: Option<crate::models::PlayerInfoDto>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::PlayerInfoDto>>,
    #[serde(rename = "rosterId", skip_serializing_if = "Option::is_none")]
    pub roster_id: Option<String>,
}

impl MatchedPlayerDto {
    pub fn new() -> MatchedPlayerDto {
        MatchedPlayerDto {
            captain: None,
            players: None,
            roster_id: None,
        }
    }
}


