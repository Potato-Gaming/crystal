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
pub struct LolPerksPerkBook {
    #[serde(rename = "currentPageId", skip_serializing_if = "Option::is_none")]
    pub current_page_id: Option<i32>,
    #[serde(rename = "pages", skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<crate::models::LolPerksPerkPageResource>>,
}

impl LolPerksPerkBook {
    pub fn new() -> LolPerksPerkBook {
        LolPerksPerkBook {
            current_page_id: None,
            pages: None,
        }
    }
}


