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
pub struct ClientConfigClientConfig {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<crate::models::ClientConfigConfigParams>,
    #[serde(rename = "updateTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
}

impl ClientConfigClientConfig {
    pub fn new() -> ClientConfigClientConfig {
        ClientConfigClientConfig {
            data: None,
            params: None,
            update_time: None,
        }
    }
}


