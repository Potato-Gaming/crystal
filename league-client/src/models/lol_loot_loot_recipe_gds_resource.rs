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
pub struct LolLootLootRecipeGdsResource {
    #[serde(rename = "contextMenuText", skip_serializing_if = "Option::is_none")]
    pub context_menu_text: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "imagePath", skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(rename = "introVideoPath", skip_serializing_if = "Option::is_none")]
    pub intro_video_path: Option<String>,
    #[serde(rename = "loopVideoPath", skip_serializing_if = "Option::is_none")]
    pub loop_video_path: Option<String>,
    #[serde(rename = "outroVideoPath", skip_serializing_if = "Option::is_none")]
    pub outro_video_path: Option<String>,
    #[serde(rename = "requirementText", skip_serializing_if = "Option::is_none")]
    pub requirement_text: Option<String>,
}

impl LolLootLootRecipeGdsResource {
    pub fn new() -> LolLootLootRecipeGdsResource {
        LolLootLootRecipeGdsResource {
            context_menu_text: None,
            description: None,
            id: None,
            image_path: None,
            intro_video_path: None,
            loop_video_path: None,
            outro_video_path: None,
            requirement_text: None,
        }
    }
}


