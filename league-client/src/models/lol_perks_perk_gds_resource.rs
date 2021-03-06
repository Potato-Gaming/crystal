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
pub struct LolPerksPerkGdsResource {
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "longDesc", skip_serializing_if = "Option::is_none")]
    pub long_desc: Option<String>,
    #[serde(rename = "majorChangePatchVersion", skip_serializing_if = "Option::is_none")]
    pub major_change_patch_version: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shortDesc", skip_serializing_if = "Option::is_none")]
    pub short_desc: Option<String>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

impl LolPerksPerkGdsResource {
    pub fn new() -> LolPerksPerkGdsResource {
        LolPerksPerkGdsResource {
            icon_path: None,
            id: None,
            long_desc: None,
            major_change_patch_version: None,
            name: None,
            short_desc: None,
            tooltip: None,
        }
    }
}


