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
pub struct LolEndOfGameSummoner {
    #[serde(rename = "summonerLevel", skip_serializing_if = "Option::is_none")]
    pub summoner_level: Option<i32>,
    #[serde(rename = "xpSinceLastLevel", skip_serializing_if = "Option::is_none")]
    pub xp_since_last_level: Option<i64>,
    #[serde(rename = "xpUntilNextLevel", skip_serializing_if = "Option::is_none")]
    pub xp_until_next_level: Option<i64>,
}

impl LolEndOfGameSummoner {
    pub fn new() -> LolEndOfGameSummoner {
        LolEndOfGameSummoner {
            summoner_level: None,
            xp_since_last_level: None,
            xp_until_next_level: None,
        }
    }
}


