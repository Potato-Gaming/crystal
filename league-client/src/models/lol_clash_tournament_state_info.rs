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
pub struct LolClashTournamentStateInfo {
    #[serde(rename = "currentPhaseId", skip_serializing_if = "Option::is_none")]
    pub current_phase_id: Option<i64>,
    #[serde(rename = "nextPhaseId", skip_serializing_if = "Option::is_none")]
    pub next_phase_id: Option<i64>,
    #[serde(rename = "nextStateChangeTime", skip_serializing_if = "Option::is_none")]
    pub next_state_change_time: Option<i64>,
    #[serde(rename = "numRemainingPeriods", skip_serializing_if = "Option::is_none")]
    pub num_remaining_periods: Option<i32>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolClashTournamentState>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
}

impl LolClashTournamentStateInfo {
    pub fn new() -> LolClashTournamentStateInfo {
        LolClashTournamentStateInfo {
            current_phase_id: None,
            next_phase_id: None,
            next_state_change_time: None,
            num_remaining_periods: None,
            state: None,
            tournament_id: None,
        }
    }
}


