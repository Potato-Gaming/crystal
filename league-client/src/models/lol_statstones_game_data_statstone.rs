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
pub struct LolStatstonesGameDataStatstone {
    #[serde(rename = "boundChampion", skip_serializing_if = "Option::is_none")]
    pub bound_champion: Option<crate::models::LolStatstonesGameDataItemReference>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "contentId", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconFull", skip_serializing_if = "Option::is_none")]
    pub icon_full: Option<String>,
    #[serde(rename = "iconLit", skip_serializing_if = "Option::is_none")]
    pub icon_lit: Option<String>,
    #[serde(rename = "iconUnlit", skip_serializing_if = "Option::is_none")]
    pub icon_unlit: Option<String>,
    #[serde(rename = "iconUnowned", skip_serializing_if = "Option::is_none")]
    pub icon_unowned: Option<String>,
    #[serde(rename = "isEpic", skip_serializing_if = "Option::is_none")]
    pub is_epic: Option<bool>,
    #[serde(rename = "isRetired", skip_serializing_if = "Option::is_none")]
    pub is_retired: Option<bool>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<i32>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "trackingType", skip_serializing_if = "Option::is_none")]
    pub tracking_type: Option<i32>,
}

impl LolStatstonesGameDataStatstone {
    pub fn new() -> LolStatstonesGameDataStatstone {
        LolStatstonesGameDataStatstone {
            bound_champion: None,
            category: None,
            content_id: None,
            description: None,
            icon_full: None,
            icon_lit: None,
            icon_unlit: None,
            icon_unowned: None,
            is_epic: None,
            is_retired: None,
            item_id: None,
            milestones: None,
            name: None,
            tracking_type: None,
        }
    }
}


