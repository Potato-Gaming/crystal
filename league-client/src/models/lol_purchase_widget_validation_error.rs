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
pub struct LolPurchaseWidgetValidationError {
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorDetails", skip_serializing_if = "Option::is_none")]
    pub error_details: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "responseItems", skip_serializing_if = "Option::is_none")]
    pub response_items: Option<Vec<String>>,
}

impl LolPurchaseWidgetValidationError {
    pub fn new() -> LolPurchaseWidgetValidationError {
        LolPurchaseWidgetValidationError {
            error_code: None,
            error_details: None,
            message: None,
            response_items: None,
        }
    }
}


