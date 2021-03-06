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
pub struct MemberBanUnbanTournament {
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
    #[serde(rename = "tournamentPhaseLockInTime", skip_serializing_if = "Option::is_none")]
    pub tournament_phase_lock_in_time: Option<i64>,
    #[serde(rename = "tournamentnameLocKey", skip_serializing_if = "Option::is_none")]
    pub tournamentname_loc_key: Option<String>,
    #[serde(rename = "tournamentnameLocKeySecondary", skip_serializing_if = "Option::is_none")]
    pub tournamentname_loc_key_secondary: Option<String>,
}

impl MemberBanUnbanTournament {
    pub fn new() -> MemberBanUnbanTournament {
        MemberBanUnbanTournament {
            tournament_id: None,
            tournament_phase_lock_in_time: None,
            tournamentname_loc_key: None,
            tournamentname_loc_key_secondary: None,
        }
    }
}


