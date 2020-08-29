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
pub struct LolCareerStatsExpertPlayer {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "expertRank", skip_serializing_if = "Option::is_none")]
    pub expert_rank: Option<i32>,
    #[serde(rename = "numOfGames", skip_serializing_if = "Option::is_none")]
    pub num_of_games: Option<i32>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<crate::models::LolCareerStatsSummonersRiftPosition>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
    #[serde(rename = "winRate", skip_serializing_if = "Option::is_none")]
    pub win_rate: Option<f32>,
}

impl LolCareerStatsExpertPlayer {
    pub fn new() -> LolCareerStatsExpertPlayer {
        LolCareerStatsExpertPlayer {
            champion_id: None,
            expert_rank: None,
            num_of_games: None,
            position: None,
            summoner_id: None,
            summoner_name: None,
            win_rate: None,
        }
    }
}

