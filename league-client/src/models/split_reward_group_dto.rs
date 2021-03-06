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
pub struct SplitRewardGroupDto {
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::SplitRewardDto>>,
    #[serde(rename = "splitPoints", skip_serializing_if = "Option::is_none")]
    pub split_points: Option<i32>,
}

impl SplitRewardGroupDto {
    pub fn new() -> SplitRewardGroupDto {
        SplitRewardGroupDto {
            rewards: None,
            split_points: None,
        }
    }
}


