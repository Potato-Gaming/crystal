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
pub struct LolClubsPublicPlayerClubMembership {
    #[serde(rename = "activeClubs", skip_serializing_if = "Option::is_none")]
    pub active_clubs: Option<Vec<crate::models::LolClubsPublicPlayerClub>>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::models::LolClubsPublicClubPlayer>,
    #[serde(rename = "preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<crate::models::LolClubsPublicClubPreferences>,
    #[serde(rename = "secureClubPresenceInfoString", skip_serializing_if = "Option::is_none")]
    pub secure_club_presence_info_string: Option<String>,
}

impl LolClubsPublicPlayerClubMembership {
    pub fn new() -> LolClubsPublicPlayerClubMembership {
        LolClubsPublicPlayerClubMembership {
            active_clubs: None,
            info: None,
            preferences: None,
            secure_club_presence_info_string: None,
        }
    }
}


