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
pub struct LolRankedRankedStats {
    #[serde(rename = "earnedRegaliaRewardIds", skip_serializing_if = "Option::is_none")]
    pub earned_regalia_reward_ids: Option<Vec<String>>,
    #[serde(rename = "highestPreviousSeasonAchievedDivision", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_achieved_division: Option<crate::models::LolRankedLeagueDivision>,
    #[serde(rename = "highestPreviousSeasonAchievedTier", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_achieved_tier: Option<crate::models::LolRankedLeagueTier>,
    #[serde(rename = "highestPreviousSeasonEndDivision", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_end_division: Option<crate::models::LolRankedLeagueDivision>,
    #[serde(rename = "highestPreviousSeasonEndTier", skip_serializing_if = "Option::is_none")]
    pub highest_previous_season_end_tier: Option<crate::models::LolRankedLeagueTier>,
    #[serde(rename = "highestRankedEntry", skip_serializing_if = "Option::is_none")]
    pub highest_ranked_entry: Option<crate::models::LolRankedRankedQueueStats>,
    #[serde(rename = "highestRankedEntrySR", skip_serializing_if = "Option::is_none")]
    pub highest_ranked_entry_sr: Option<crate::models::LolRankedRankedQueueStats>,
    #[serde(rename = "queueMap", skip_serializing_if = "Option::is_none")]
    pub queue_map: Option<::std::collections::HashMap<String, crate::models::LolRankedRankedQueueStats>>,
    #[serde(rename = "queues", skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<crate::models::LolRankedRankedQueueStats>>,
    #[serde(rename = "rankedRegaliaLevel", skip_serializing_if = "Option::is_none")]
    pub ranked_regalia_level: Option<i32>,
    #[serde(rename = "splitsProgress", skip_serializing_if = "Option::is_none")]
    pub splits_progress: Option<::std::collections::HashMap<String, i32>>,
}

impl LolRankedRankedStats {
    pub fn new() -> LolRankedRankedStats {
        LolRankedRankedStats {
            earned_regalia_reward_ids: None,
            highest_previous_season_achieved_division: None,
            highest_previous_season_achieved_tier: None,
            highest_previous_season_end_division: None,
            highest_previous_season_end_tier: None,
            highest_ranked_entry: None,
            highest_ranked_entry_sr: None,
            queue_map: None,
            queues: None,
            ranked_regalia_level: None,
            splits_progress: None,
        }
    }
}


