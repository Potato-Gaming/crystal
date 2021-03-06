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
pub struct GcloudVoiceChatConfigResource {
    #[serde(rename = "accessTokenUri", skip_serializing_if = "Option::is_none")]
    pub access_token_uri: Option<String>,
    #[serde(rename = "authTokenUri", skip_serializing_if = "Option::is_none")]
    pub auth_token_uri: Option<String>,
    #[serde(rename = "jwt2gvtUrl", skip_serializing_if = "Option::is_none")]
    pub jwt2gvt_url: Option<String>,
    #[serde(rename = "logLevel", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<i32>,
    #[serde(rename = "logLevelName", skip_serializing_if = "Option::is_none")]
    pub log_level_name: Option<String>,
    #[serde(rename = "logsPath", skip_serializing_if = "Option::is_none")]
    pub logs_path: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "useExternalAuth", skip_serializing_if = "Option::is_none")]
    pub use_external_auth: Option<bool>,
    #[serde(rename = "voiceDomain", skip_serializing_if = "Option::is_none")]
    pub voice_domain: Option<String>,
    #[serde(rename = "voiceServerUri", skip_serializing_if = "Option::is_none")]
    pub voice_server_uri: Option<String>,
}

impl GcloudVoiceChatConfigResource {
    pub fn new() -> GcloudVoiceChatConfigResource {
        GcloudVoiceChatConfigResource {
            access_token_uri: None,
            auth_token_uri: None,
            jwt2gvt_url: None,
            log_level: None,
            log_level_name: None,
            logs_path: None,
            provider: None,
            use_external_auth: None,
            voice_domain: None,
            voice_server_uri: None,
        }
    }
}


