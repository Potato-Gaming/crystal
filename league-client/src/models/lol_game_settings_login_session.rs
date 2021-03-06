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
pub struct LolGameSettingsLoginSession {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "gasToken", skip_serializing_if = "Option::is_none")]
    pub gas_token: Option<serde_json::Value>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolGameSettingsLoginSessionStates>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolGameSettingsLoginSession {
    pub fn new() -> LolGameSettingsLoginSession {
        LolGameSettingsLoginSession {
            account_id: None,
            gas_token: None,
            state: None,
            summoner_id: None,
        }
    }
}


