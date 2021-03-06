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
pub struct LolItemSetsItemSet {
    #[serde(rename = "associatedChampions", skip_serializing_if = "Option::is_none")]
    pub associated_champions: Option<Vec<i32>>,
    #[serde(rename = "associatedMaps", skip_serializing_if = "Option::is_none")]
    pub associated_maps: Option<Vec<i32>>,
    #[serde(rename = "blocks", skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<crate::models::LolItemSetsItemSetBlock>>,
    #[serde(rename = "map", skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "preferredItemSlots", skip_serializing_if = "Option::is_none")]
    pub preferred_item_slots: Option<Vec<crate::models::LolItemSetsPreferredItemSlot>>,
    #[serde(rename = "sortrank", skip_serializing_if = "Option::is_none")]
    pub sortrank: Option<i32>,
    #[serde(rename = "startedFrom", skip_serializing_if = "Option::is_none")]
    pub started_from: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl LolItemSetsItemSet {
    pub fn new() -> LolItemSetsItemSet {
        LolItemSetsItemSet {
            associated_champions: None,
            associated_maps: None,
            blocks: None,
            map: None,
            mode: None,
            preferred_item_slots: None,
            sortrank: None,
            started_from: None,
            title: None,
            _type: None,
            uid: None,
        }
    }
}


