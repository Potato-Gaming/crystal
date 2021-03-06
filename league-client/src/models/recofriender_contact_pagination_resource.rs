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
pub struct RecofrienderContactPaginationResource {
    #[serde(rename = "cached", skip_serializing_if = "Option::is_none")]
    pub cached: Option<i64>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "more", skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl RecofrienderContactPaginationResource {
    pub fn new() -> RecofrienderContactPaginationResource {
        RecofrienderContactPaginationResource {
            cached: None,
            count: None,
            limit: None,
            more: None,
            start: None,
            total: None,
        }
    }
}


