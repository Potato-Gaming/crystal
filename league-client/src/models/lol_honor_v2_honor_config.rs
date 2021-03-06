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
pub struct LolHonorV2HonorConfig {
    #[serde(rename = "DayOneModalEnabled", skip_serializing_if = "Option::is_none")]
    pub day_one_modal_enabled: Option<bool>,
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Honor2018Enabled", skip_serializing_if = "Option::is_none")]
    pub honor2018_enabled: Option<bool>,
    #[serde(rename = "SecondsToVote", skip_serializing_if = "Option::is_none")]
    pub seconds_to_vote: Option<i32>,
}

impl LolHonorV2HonorConfig {
    pub fn new() -> LolHonorV2HonorConfig {
        LolHonorV2HonorConfig {
            day_one_modal_enabled: None,
            enabled: None,
            honor2018_enabled: None,
            seconds_to_vote: None,
        }
    }
}


