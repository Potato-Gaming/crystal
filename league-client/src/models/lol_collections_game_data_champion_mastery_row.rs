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
pub struct LolCollectionsGameDataChampionMasteryRow {
    #[serde(rename = "masteries", skip_serializing_if = "Option::is_none")]
    pub masteries: Option<Vec<i32>>,
}

impl LolCollectionsGameDataChampionMasteryRow {
    pub fn new() -> LolCollectionsGameDataChampionMasteryRow {
        LolCollectionsGameDataChampionMasteryRow {
            masteries: None,
        }
    }
}


