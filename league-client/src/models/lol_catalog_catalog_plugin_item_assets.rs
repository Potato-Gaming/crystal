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
pub struct LolCatalogCatalogPluginItemAssets {
    #[serde(rename = "colors", skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    #[serde(rename = "emblems", skip_serializing_if = "Option::is_none")]
    pub emblems: Option<Vec<crate::models::LolCatalogChampionSkinEmblem>>,
    #[serde(rename = "iconPath", skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    #[serde(rename = "splashPath", skip_serializing_if = "Option::is_none")]
    pub splash_path: Option<String>,
    #[serde(rename = "tilePath", skip_serializing_if = "Option::is_none")]
    pub tile_path: Option<String>,
}

impl LolCatalogCatalogPluginItemAssets {
    pub fn new() -> LolCatalogCatalogPluginItemAssets {
        LolCatalogCatalogPluginItemAssets {
            colors: None,
            emblems: None,
            icon_path: None,
            splash_path: None,
            tile_path: None,
        }
    }
}


