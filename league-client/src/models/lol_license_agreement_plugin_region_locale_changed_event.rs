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
pub struct LolLicenseAgreementPluginRegionLocaleChangedEvent {
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl LolLicenseAgreementPluginRegionLocaleChangedEvent {
    pub fn new() -> LolLicenseAgreementPluginRegionLocaleChangedEvent {
        LolLicenseAgreementPluginRegionLocaleChangedEvent {
            locale: None,
            region: None,
        }
    }
}


