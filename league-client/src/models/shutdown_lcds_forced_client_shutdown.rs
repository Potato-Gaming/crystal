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
pub struct ShutdownLcdsForcedClientShutdown {
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ShutdownLcdsForcedClientShutdown {
    pub fn new() -> ShutdownLcdsForcedClientShutdown {
        ShutdownLcdsForcedClientShutdown {
            additional_info: None,
            reason: None,
        }
    }
}


