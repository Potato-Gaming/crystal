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
pub struct LolTrophiesInventoryResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<crate::models::LolTrophiesInventoryItemsByType>,
}

impl LolTrophiesInventoryResponse {
    pub fn new() -> LolTrophiesInventoryResponse {
        LolTrophiesInventoryResponse {
            items: None,
        }
    }
}


