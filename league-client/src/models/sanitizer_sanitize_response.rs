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
pub struct SanitizerSanitizeResponse {
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<bool>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "texts", skip_serializing_if = "Option::is_none")]
    pub texts: Option<Vec<String>>,
}

impl SanitizerSanitizeResponse {
    pub fn new() -> SanitizerSanitizeResponse {
        SanitizerSanitizeResponse {
            modified: None,
            text: None,
            texts: None,
        }
    }
}


