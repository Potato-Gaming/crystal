/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// BindingFullTypeHelp : Describes a struct or enum type.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BindingFullTypeHelp {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::BindingFullFieldHelp>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameSpace", skip_serializing_if = "Option::is_none")]
    pub name_space: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<crate::models::BindingFullEnumValueHelp>>,
}

impl BindingFullTypeHelp {
    /// Describes a struct or enum type.
    pub fn new() -> BindingFullTypeHelp {
        BindingFullTypeHelp {
            description: None,
            fields: None,
            name: None,
            name_space: None,
            size: None,
            tags: None,
            values: None,
        }
    }
}


