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
pub struct LolAccountVerificationAvsConfig {
    #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PasswordEnabled", skip_serializing_if = "Option::is_none")]
    pub password_enabled: Option<bool>,
}

impl LolAccountVerificationAvsConfig {
    pub fn new() -> LolAccountVerificationAvsConfig {
        LolAccountVerificationAvsConfig {
            enabled: None,
            password_enabled: None,
        }
    }
}


