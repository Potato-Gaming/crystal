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
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournaments {
    #[serde(rename = "brackets", skip_serializing_if = "Option::is_none")]
    pub brackets: Option<::std::collections::HashMap<String, crate::models::LolEsportStreamNotificationsEsportsApiHighlanderTournamentsBrackets>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "rosters", skip_serializing_if = "Option::is_none")]
    pub rosters: Option<::std::collections::HashMap<String, crate::models::LolEsportStreamNotificationsEsportsApiHighlanderTournamentsRosters>>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl LolEsportStreamNotificationsEsportsApiHighlanderTournaments {
    pub fn new() -> LolEsportStreamNotificationsEsportsApiHighlanderTournaments {
        LolEsportStreamNotificationsEsportsApiHighlanderTournaments {
            brackets: None,
            description: None,
            id: None,
            rosters: None,
            title: None,
        }
    }
}

