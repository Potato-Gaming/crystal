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
pub struct LolStoreGiftableResult {
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::models::LolStoreGiftingConfig>,
    #[serde(rename = "friends", skip_serializing_if = "Option::is_none")]
    pub friends: Option<Vec<crate::models::LolStoreGiftingFriend>>,
}

impl LolStoreGiftableResult {
    pub fn new() -> LolStoreGiftableResult {
        LolStoreGiftableResult {
            config: None,
            friends: None,
        }
    }
}


