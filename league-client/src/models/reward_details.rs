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
pub struct RewardDetails {
    #[serde(rename = "rosterId", skip_serializing_if = "Option::is_none")]
    pub roster_id: Option<i64>,
    #[serde(rename = "teamMemberIds", skip_serializing_if = "Option::is_none")]
    pub team_member_ids: Option<Vec<i64>>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
}

impl RewardDetails {
    pub fn new() -> RewardDetails {
        RewardDetails {
            roster_id: None,
            team_member_ids: None,
            tournament_id: None,
        }
    }
}

