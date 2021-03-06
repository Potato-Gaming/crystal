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
pub struct LolCatalogGameDataStatstone {
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconFull", skip_serializing_if = "Option::is_none")]
    pub icon_full: Option<String>,
    #[serde(rename = "isDuration", skip_serializing_if = "Option::is_none")]
    pub is_duration: Option<bool>,
    #[serde(rename = "isEpic", skip_serializing_if = "Option::is_none")]
    pub is_epic: Option<bool>,
    #[serde(rename = "isRetired", skip_serializing_if = "Option::is_none")]
    pub is_retired: Option<bool>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LolCatalogGameDataStatstone {
    pub fn new() -> LolCatalogGameDataStatstone {
        LolCatalogGameDataStatstone {
            category: None,
            content_id: None,
            description: None,
            icon_full: None,
            is_duration: None,
            is_epic: None,
            is_retired: None,
            item_id: None,
            name: None,
        }
    }
}


