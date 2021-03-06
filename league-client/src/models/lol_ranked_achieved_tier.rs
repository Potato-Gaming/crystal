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
pub struct LolRankedAchievedTier {
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<i64>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<crate::models::LolRankedLeagueQueueType>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<crate::models::LolRankedLeagueTier>,
}

impl LolRankedAchievedTier {
    pub fn new() -> LolRankedAchievedTier {
        LolRankedAchievedTier {
            division: None,
            queue_type: None,
            tier: None,
        }
    }
}


