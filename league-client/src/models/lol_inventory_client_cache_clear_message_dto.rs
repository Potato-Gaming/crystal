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
pub struct LolInventoryClientCacheClearMessageDto {
    #[serde(rename = "clearAll", skip_serializing_if = "Option::is_none")]
    pub clear_all: Option<bool>,
    #[serde(rename = "inventoryTypes", skip_serializing_if = "Option::is_none")]
    pub inventory_types: Option<Vec<String>>,
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

impl LolInventoryClientCacheClearMessageDto {
    pub fn new() -> LolInventoryClientCacheClearMessageDto {
        LolInventoryClientCacheClearMessageDto {
            clear_all: None,
            inventory_types: None,
            regions: None,
        }
    }
}

