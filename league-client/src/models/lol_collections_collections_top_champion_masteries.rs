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
pub struct LolCollectionsCollectionsTopChampionMasteries {
    #[serde(rename = "masteries", skip_serializing_if = "Option::is_none")]
    pub masteries: Option<Vec<crate::models::LolCollectionsCollectionsChampionMastery>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolCollectionsCollectionsTopChampionMasteries {
    pub fn new() -> LolCollectionsCollectionsTopChampionMasteries {
        LolCollectionsCollectionsTopChampionMasteries {
            masteries: None,
            score: None,
            summoner_id: None,
        }
    }
}

