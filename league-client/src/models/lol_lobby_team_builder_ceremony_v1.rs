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
pub struct LolLobbyTeamBuilderCeremonyV1 {
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LolLobbyTeamBuilderCeremonyV1 {
    pub fn new() -> LolLobbyTeamBuilderCeremonyV1 {
        LolLobbyTeamBuilderCeremonyV1 {
            duration: None,
            name: None,
        }
    }
}


