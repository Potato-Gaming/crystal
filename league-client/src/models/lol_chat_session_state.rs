/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LolChatSessionState {
    #[serde(rename = "initializing")]
    Initializing,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "loaded")]
    Loaded,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "shuttingdown")]
    Shuttingdown,

}




