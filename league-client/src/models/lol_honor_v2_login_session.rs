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
pub struct LolHonorV2LoginSession {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolHonorV2LoginSessionStates>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolHonorV2LoginSession {
    pub fn new() -> LolHonorV2LoginSession {
        LolHonorV2LoginSession {
            account_id: None,
            state: None,
            summoner_id: None,
        }
    }
}

