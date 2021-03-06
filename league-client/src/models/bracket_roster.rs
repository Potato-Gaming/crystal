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
pub struct BracketRoster {
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<i32>,
    #[serde(rename = "logoColor", skip_serializing_if = "Option::is_none")]
    pub logo_color: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rosterId", skip_serializing_if = "Option::is_none")]
    pub roster_id: Option<i64>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
}

impl BracketRoster {
    pub fn new() -> BracketRoster {
        BracketRoster {
            logo: None,
            logo_color: None,
            name: None,
            roster_id: None,
            short_name: None,
        }
    }
}


