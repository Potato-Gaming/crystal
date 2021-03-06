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
pub struct LolRankedSplitPointsNotification {
    #[serde(rename = "nextRewardId", skip_serializing_if = "Option::is_none")]
    pub next_reward_id: Option<String>,
    #[serde(rename = "nextRewardType", skip_serializing_if = "Option::is_none")]
    pub next_reward_type: Option<String>,
    #[serde(rename = "previousSplitPointsRequired", skip_serializing_if = "Option::is_none")]
    pub previous_split_points_required: Option<i32>,
    #[serde(rename = "splitPointsAfterGame", skip_serializing_if = "Option::is_none")]
    pub split_points_after_game: Option<i32>,
    #[serde(rename = "splitPointsBeforeGame", skip_serializing_if = "Option::is_none")]
    pub split_points_before_game: Option<i32>,
    #[serde(rename = "splitPointsBreakdown", skip_serializing_if = "Option::is_none")]
    pub split_points_breakdown: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "splitPointsDelta", skip_serializing_if = "Option::is_none")]
    pub split_points_delta: Option<i32>,
    #[serde(rename = "splitPointsRequired", skip_serializing_if = "Option::is_none")]
    pub split_points_required: Option<i32>,
}

impl LolRankedSplitPointsNotification {
    pub fn new() -> LolRankedSplitPointsNotification {
        LolRankedSplitPointsNotification {
            next_reward_id: None,
            next_reward_type: None,
            previous_split_points_required: None,
            split_points_after_game: None,
            split_points_before_game: None,
            split_points_breakdown: None,
            split_points_delta: None,
            split_points_required: None,
        }
    }
}


