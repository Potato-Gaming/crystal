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
pub struct LolInventoryInventoryItem {
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "ownershipType", skip_serializing_if = "Option::is_none")]
    pub ownership_type: Option<crate::models::LolInventoryItemOwnershipType>,
    #[serde(rename = "purchaseDate", skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl LolInventoryInventoryItem {
    pub fn new() -> LolInventoryInventoryItem {
        LolInventoryInventoryItem {
            inventory_type: None,
            item_id: None,
            ownership_type: None,
            purchase_date: None,
            quantity: None,
            uuid: None,
        }
    }
}


