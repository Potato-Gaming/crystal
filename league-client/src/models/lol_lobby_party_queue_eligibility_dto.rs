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
pub struct LolLobbyPartyQueueEligibilityDto {
    #[serde(rename = "availableQueueIds", skip_serializing_if = "Option::is_none")]
    pub available_queue_ids: Option<Vec<i32>>,
    #[serde(rename = "partyRestrictions", skip_serializing_if = "Option::is_none")]
    pub party_restrictions: Option<Vec<crate::models::LolLobbyGatekeeperRestrictionDto>>,
}

impl LolLobbyPartyQueueEligibilityDto {
    pub fn new() -> LolLobbyPartyQueueEligibilityDto {
        LolLobbyPartyQueueEligibilityDto {
            available_queue_ids: None,
            party_restrictions: None,
        }
    }
}

