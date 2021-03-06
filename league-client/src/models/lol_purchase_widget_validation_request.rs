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
pub struct LolPurchaseWidgetValidationRequest {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolPurchaseWidgetValidationRequestItem>>,
    #[serde(rename = "ownedItems", skip_serializing_if = "Option::is_none")]
    pub owned_items: Option<Vec<crate::models::LolPurchaseWidgetItemOwnership>>,
}

impl LolPurchaseWidgetValidationRequest {
    pub fn new() -> LolPurchaseWidgetValidationRequest {
        LolPurchaseWidgetValidationRequest {
            items: None,
            owned_items: None,
        }
    }
}


