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
pub struct LolLobbyPartyMemberDto {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "canInvite", skip_serializing_if = "Option::is_none")]
    pub can_invite: Option<bool>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<crate::models::LolLobbyGameModeDto>,
    #[serde(rename = "inviteTimestamp", skip_serializing_if = "Option::is_none")]
    pub invite_timestamp: Option<i64>,
    #[serde(rename = "invitedBySummonerId", skip_serializing_if = "Option::is_none")]
    pub invited_by_summoner_id: Option<i64>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::models::LolLobbyPartyMemberMetadataDto>,
    #[serde(rename = "partyId", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(rename = "partyVersion", skip_serializing_if = "Option::is_none")]
    pub party_version: Option<i64>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::LolLobbyPartyMemberRoleEnum>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolLobbyPartyMemberDto {
    pub fn new() -> LolLobbyPartyMemberDto {
        LolLobbyPartyMemberDto {
            account_id: None,
            can_invite: None,
            game_mode: None,
            invite_timestamp: None,
            invited_by_summoner_id: None,
            metadata: None,
            party_id: None,
            party_version: None,
            platform_id: None,
            puuid: None,
            ready: None,
            role: None,
            summoner_id: None,
        }
    }
}


