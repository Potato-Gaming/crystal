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
pub struct LolPreEndOfGameSequenceEvent {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

impl LolPreEndOfGameSequenceEvent {
    pub fn new() -> LolPreEndOfGameSequenceEvent {
        LolPreEndOfGameSequenceEvent {
            name: None,
            priority: None,
        }
    }
}


