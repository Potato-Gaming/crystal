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
pub struct LolSummonerSummonerIcon {
    #[serde(rename = "inventoryToken", skip_serializing_if = "Option::is_none")]
    pub inventory_token: Option<String>,
    #[serde(rename = "profileIconId", skip_serializing_if = "Option::is_none")]
    pub profile_icon_id: Option<i32>,
}

impl LolSummonerSummonerIcon {
    pub fn new() -> LolSummonerSummonerIcon {
        LolSummonerSummonerIcon {
            inventory_token: None,
            profile_icon_id: None,
        }
    }
}


