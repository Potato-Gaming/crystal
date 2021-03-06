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
pub struct LolNpeRewardsAccountSettingsData {
    #[serde(rename = "challenges", skip_serializing_if = "Option::is_none")]
    pub challenges: Option<crate::models::LolNpeRewardsChallengesSettings>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<crate::models::LolNpeRewardsLoginSeriesSettings>,
}

impl LolNpeRewardsAccountSettingsData {
    pub fn new() -> LolNpeRewardsAccountSettingsData {
        LolNpeRewardsAccountSettingsData {
            challenges: None,
            login: None,
        }
    }
}


