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
pub struct LolBannersBannerFrame {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}

impl LolBannersBannerFrame {
    pub fn new() -> LolBannersBannerFrame {
        LolBannersBannerFrame {
            level: None,
        }
    }
}

