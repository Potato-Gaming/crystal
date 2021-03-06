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
pub struct LootLcdsRecipeMetadata {
    #[serde(rename = "bonusDescriptions", skip_serializing_if = "Option::is_none")]
    pub bonus_descriptions: Option<Vec<crate::models::LootLcdsLootDescriptionDto>>,
    #[serde(rename = "guaranteedDescriptions", skip_serializing_if = "Option::is_none")]
    pub guaranteed_descriptions: Option<Vec<crate::models::LootLcdsLootDescriptionDto>>,
    #[serde(rename = "tooltipsDisabled", skip_serializing_if = "Option::is_none")]
    pub tooltips_disabled: Option<bool>,
}

impl LootLcdsRecipeMetadata {
    pub fn new() -> LootLcdsRecipeMetadata {
        LootLcdsRecipeMetadata {
            bonus_descriptions: None,
            guaranteed_descriptions: None,
            tooltips_disabled: None,
        }
    }
}


