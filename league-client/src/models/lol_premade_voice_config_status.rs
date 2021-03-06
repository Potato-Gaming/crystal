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
pub struct LolPremadeVoiceConfigStatus {
    #[serde(rename = "readiness", skip_serializing_if = "Option::is_none")]
    pub readiness: Option<crate::models::LolPremadeVoiceConfigReadinessEnum>,
}

impl LolPremadeVoiceConfigStatus {
    pub fn new() -> LolPremadeVoiceConfigStatus {
        LolPremadeVoiceConfigStatus {
            readiness: None,
        }
    }
}


