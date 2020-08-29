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
pub struct LolLootRecipeMenuConfig {
    #[serde(rename = "alwaysShowLootIds", skip_serializing_if = "Option::is_none")]
    pub always_show_loot_ids: Option<Vec<String>>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "lootItemsUsingBreakoutRecipeMenu", skip_serializing_if = "Option::is_none")]
    pub loot_items_using_breakout_recipe_menu: Option<Vec<String>>,
}

impl LolLootRecipeMenuConfig {
    pub fn new() -> LolLootRecipeMenuConfig {
        LolLootRecipeMenuConfig {
            always_show_loot_ids: None,
            enabled: None,
            loot_items_using_breakout_recipe_menu: None,
        }
    }
}

