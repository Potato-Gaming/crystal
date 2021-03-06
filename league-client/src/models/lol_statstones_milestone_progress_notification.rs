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
pub struct LolStatstonesMilestoneProgressNotification {
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(rename = "statstoneId", skip_serializing_if = "Option::is_none")]
    pub statstone_id: Option<String>,
    #[serde(rename = "statstoneName", skip_serializing_if = "Option::is_none")]
    pub statstone_name: Option<String>,
    #[serde(rename = "threshold", skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i32>,
}

impl LolStatstonesMilestoneProgressNotification {
    pub fn new() -> LolStatstonesMilestoneProgressNotification {
        LolStatstonesMilestoneProgressNotification {
            image_url: None,
            level: None,
            statstone_id: None,
            statstone_name: None,
            threshold: None,
        }
    }
}


