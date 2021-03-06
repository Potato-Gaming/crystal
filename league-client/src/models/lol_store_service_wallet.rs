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
pub struct LolStoreServiceWallet {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<crate::models::LolStoreServiceBalance>>,
}

impl LolStoreServiceWallet {
    pub fn new() -> LolStoreServiceWallet {
        LolStoreServiceWallet {
            account_id: None,
            balances: None,
        }
    }
}


