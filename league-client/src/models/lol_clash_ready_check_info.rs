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
pub struct LolClashReadyCheckInfo {
    #[serde(rename = "acceptError", skip_serializing_if = "Option::is_none")]
    pub accept_error: Option<String>,
    #[serde(rename = "isAcceptSuccessful", skip_serializing_if = "Option::is_none")]
    pub is_accept_successful: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "readyCheckResource", skip_serializing_if = "Option::is_none")]
    pub ready_check_resource: Option<crate::models::LolClashMatchmakingReadyCheckResource>,
    #[serde(rename = "timestampLastClashGameflowDodge", skip_serializing_if = "Option::is_none")]
    pub timestamp_last_clash_gameflow_dodge: Option<i64>,
    #[serde(rename = "timestampReceived", skip_serializing_if = "Option::is_none")]
    pub timestamp_received: Option<i64>,
    #[serde(rename = "timestampResponseComplete", skip_serializing_if = "Option::is_none")]
    pub timestamp_response_complete: Option<i64>,
}

impl LolClashReadyCheckInfo {
    pub fn new() -> LolClashReadyCheckInfo {
        LolClashReadyCheckInfo {
            accept_error: None,
            is_accept_successful: None,
            queue_id: None,
            ready_check_resource: None,
            timestamp_last_clash_gameflow_dodge: None,
            timestamp_received: None,
            timestamp_response_complete: None,
        }
    }
}


