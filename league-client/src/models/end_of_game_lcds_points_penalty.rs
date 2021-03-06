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
pub struct EndOfGameLcdsPointsPenalty {
    #[serde(rename = "penalty", skip_serializing_if = "Option::is_none")]
    pub penalty: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl EndOfGameLcdsPointsPenalty {
    pub fn new() -> EndOfGameLcdsPointsPenalty {
        EndOfGameLcdsPointsPenalty {
            penalty: None,
            _type: None,
        }
    }
}


