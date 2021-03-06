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
pub struct LolStorePageDto {
    #[serde(rename = "Player", skip_serializing_if = "Option::is_none")]
    pub player: Option<crate::models::LolStorePlayer>,
    #[serde(rename = "catalog", skip_serializing_if = "Option::is_none")]
    pub catalog: Option<Vec<crate::models::LolStoreCatalogItem>>,
    #[serde(rename = "groupOrder", skip_serializing_if = "Option::is_none")]
    pub group_order: Option<Vec<String>>,
    #[serde(rename = "itemGroups", skip_serializing_if = "Option::is_none")]
    pub item_groups: Option<::std::collections::HashMap<String, crate::models::LolStorePageGroupingDto>>,
}

impl LolStorePageDto {
    pub fn new() -> LolStorePageDto {
        LolStorePageDto {
            player: None,
            catalog: None,
            group_order: None,
            item_groups: None,
        }
    }
}


