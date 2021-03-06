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
pub struct LolChatParticipantList {
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::LolChatParticipant>>,
}

impl LolChatParticipantList {
    pub fn new() -> LolChatParticipantList {
        LolChatParticipantList {
            participants: None,
        }
    }
}


