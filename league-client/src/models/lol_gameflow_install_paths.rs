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
pub struct LolGameflowInstallPaths {
    #[serde(rename = "gameExecutablePath", skip_serializing_if = "Option::is_none")]
    pub game_executable_path: Option<String>,
    #[serde(rename = "gameInstallRoot", skip_serializing_if = "Option::is_none")]
    pub game_install_root: Option<String>,
}

impl LolGameflowInstallPaths {
    pub fn new() -> LolGameflowInstallPaths {
        LolGameflowInstallPaths {
            game_executable_path: None,
            game_install_root: None,
        }
    }
}


