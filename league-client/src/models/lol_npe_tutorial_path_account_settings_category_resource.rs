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
pub struct LolNpeTutorialPathAccountSettingsCategoryResource {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::models::LolNpeTutorialPathAccountSettingsTutorial>,
    #[serde(rename = "schemaVersion", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
}

impl LolNpeTutorialPathAccountSettingsCategoryResource {
    pub fn new() -> LolNpeTutorialPathAccountSettingsCategoryResource {
        LolNpeTutorialPathAccountSettingsCategoryResource {
            data: None,
            schema_version: None,
        }
    }
}

