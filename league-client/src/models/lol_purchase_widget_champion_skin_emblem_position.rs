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
pub struct LolPurchaseWidgetChampionSkinEmblemPosition {
    #[serde(rename = "horizontal", skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<String>,
    #[serde(rename = "vertical", skip_serializing_if = "Option::is_none")]
    pub vertical: Option<String>,
}

impl LolPurchaseWidgetChampionSkinEmblemPosition {
    pub fn new() -> LolPurchaseWidgetChampionSkinEmblemPosition {
        LolPurchaseWidgetChampionSkinEmblemPosition {
            horizontal: None,
            vertical: None,
        }
    }
}


