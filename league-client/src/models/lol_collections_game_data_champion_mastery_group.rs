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
pub struct LolCollectionsGameDataChampionMasteryGroup {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<crate::models::LolCollectionsGameDataChampionMasteryRow>>,
}

impl LolCollectionsGameDataChampionMasteryGroup {
    pub fn new() -> LolCollectionsGameDataChampionMasteryGroup {
        LolCollectionsGameDataChampionMasteryGroup {
            id: None,
            rows: None,
        }
    }
}

