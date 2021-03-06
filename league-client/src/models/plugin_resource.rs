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
pub struct PluginResource {
    #[serde(rename = "app", skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "assetBundleNames", skip_serializing_if = "Option::is_none")]
    pub asset_bundle_names: Option<Vec<String>>,
    #[serde(rename = "dependencies", skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<crate::models::PluginResourceContract>>,
    #[serde(rename = "feature", skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "implementedContracts", skip_serializing_if = "Option::is_none")]
    pub implemented_contracts: Option<Vec<crate::models::PluginResourceContract>>,
    #[serde(rename = "mountedAssetBundles", skip_serializing_if = "Option::is_none")]
    pub mounted_asset_bundles: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "orderWADFileMounted", skip_serializing_if = "Option::is_none")]
    pub order_wad_file_mounted: Option<i32>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "subtype", skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "supertype", skip_serializing_if = "Option::is_none")]
    pub supertype: Option<String>,
    #[serde(rename = "threadingModel", skip_serializing_if = "Option::is_none")]
    pub threading_model: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl PluginResource {
    pub fn new() -> PluginResource {
        PluginResource {
            app: None,
            asset_bundle_names: None,
            dependencies: None,
            feature: None,
            full_name: None,
            implemented_contracts: None,
            mounted_asset_bundles: None,
            order_wad_file_mounted: None,
            short_name: None,
            subtype: None,
            supertype: None,
            threading_model: None,
            version: None,
        }
    }
}


