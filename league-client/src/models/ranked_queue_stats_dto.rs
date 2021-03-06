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
pub struct RankedQueueStatsDto {
    #[serde(rename = "leaguePoints", skip_serializing_if = "Option::is_none")]
    pub league_points: Option<i32>,
    #[serde(rename = "losses", skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,
    #[serde(rename = "miniSeriesProgress", skip_serializing_if = "Option::is_none")]
    pub mini_series_progress: Option<String>,
    #[serde(rename = "previousSeasonAchievedRank", skip_serializing_if = "Option::is_none")]
    pub previous_season_achieved_rank: Option<String>,
    #[serde(rename = "previousSeasonAchievedTier", skip_serializing_if = "Option::is_none")]
    pub previous_season_achieved_tier: Option<String>,
    #[serde(rename = "previousSeasonEndRank", skip_serializing_if = "Option::is_none")]
    pub previous_season_end_rank: Option<String>,
    #[serde(rename = "previousSeasonEndTier", skip_serializing_if = "Option::is_none")]
    pub previous_season_end_tier: Option<String>,
    #[serde(rename = "provisionalGameThreshold", skip_serializing_if = "Option::is_none")]
    pub provisional_game_threshold: Option<i32>,
    #[serde(rename = "provisionalGamesRemaining", skip_serializing_if = "Option::is_none")]
    pub provisional_games_remaining: Option<i32>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<crate::models::RankedQueueWarningsDto>,
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,
}

impl RankedQueueStatsDto {
    pub fn new() -> RankedQueueStatsDto {
        RankedQueueStatsDto {
            league_points: None,
            losses: None,
            mini_series_progress: None,
            previous_season_achieved_rank: None,
            previous_season_achieved_tier: None,
            previous_season_end_rank: None,
            previous_season_end_tier: None,
            provisional_game_threshold: None,
            provisional_games_remaining: None,
            queue_type: None,
            rank: None,
            tier: None,
            warnings: None,
            wins: None,
        }
    }
}


