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
pub struct SplitRewardsMetaData {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

impl SplitRewardsMetaData {
    pub fn new() -> SplitRewardsMetaData {
        SplitRewardsMetaData {
            description: None,
            quantity: None,
        }
    }
}

