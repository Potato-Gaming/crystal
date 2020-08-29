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
pub struct LolChatAuthResourcePlain {
    #[serde(rename = "gasToken", skip_serializing_if = "Option::is_none")]
    pub gas_token: Option<serde_json::Value>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LolChatAuthResourcePlain {
    pub fn new() -> LolChatAuthResourcePlain {
        LolChatAuthResourcePlain {
            gas_token: None,
            password: None,
            username: None,
        }
    }
}

