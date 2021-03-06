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
pub enum LolLobbyEligibilityRestrictionCode {
    #[serde(rename = "QueueDisabled")]
    QueueDisabled,
    #[serde(rename = "QueueUnsupported")]
    QueueUnsupported,
    #[serde(rename = "PlayerLevelRestriction")]
    PlayerLevelRestriction,
    #[serde(rename = "PlayerTimedRestriction")]
    PlayerTimedRestriction,
    #[serde(rename = "PlayerBannedRestriction")]
    PlayerBannedRestriction,
    #[serde(rename = "PlayerAvailableChampionRestriction")]
    PlayerAvailableChampionRestriction,
    #[serde(rename = "TeamDivisionRestriction")]
    TeamDivisionRestriction,
    #[serde(rename = "TeamMaxSizeRestriction")]
    TeamMaxSizeRestriction,
    #[serde(rename = "TeamMinSizeRestriction")]
    TeamMinSizeRestriction,
    #[serde(rename = "PlayerBingeRestriction")]
    PlayerBingeRestriction,
    #[serde(rename = "PlayerDodgeRestriction")]
    PlayerDodgeRestriction,
    #[serde(rename = "PlayerInGameRestriction")]
    PlayerInGameRestriction,
    #[serde(rename = "PlayerLeaverBustedRestriction")]
    PlayerLeaverBustedRestriction,
    #[serde(rename = "PlayerLeaverTaintedWarningRestriction")]
    PlayerLeaverTaintedWarningRestriction,
    #[serde(rename = "PlayerMaxLevelRestriction")]
    PlayerMaxLevelRestriction,
    #[serde(rename = "PlayerMinLevelRestriction")]
    PlayerMinLevelRestriction,
    #[serde(rename = "PlayerMinorRestriction")]
    PlayerMinorRestriction,
    #[serde(rename = "PlayerTimePlayedRestriction")]
    PlayerTimePlayedRestriction,
    #[serde(rename = "PlayerRankedSuspensionRestriction")]
    PlayerRankedSuspensionRestriction,
    #[serde(rename = "TeamHighMMRMaxSizeRestriction")]
    TeamHighMMRMaxSizeRestriction,
    #[serde(rename = "TeamSizeRestriction")]
    TeamSizeRestriction,
    #[serde(rename = "PrerequisiteQueuesNotPlayedRestriction")]
    PrerequisiteQueuesNotPlayedRestriction,
    #[serde(rename = "GameVersionMismatch")]
    GameVersionMismatch,
    #[serde(rename = "GameVersionMissing")]
    GameVersionMissing,
    #[serde(rename = "GameVersionNotSupported")]
    GameVersionNotSupported,
    #[serde(rename = "QueueEntryNotEntitledRestriction")]
    QueueEntryNotEntitledRestriction,
    #[serde(rename = "UnknownRestriction")]
    UnknownRestriction,
    #[serde(rename = "BanInfoNotAvailable")]
    BanInfoNotAvailable,
    #[serde(rename = "MinorInfoNotAvailable")]
    MinorInfoNotAvailable,
    #[serde(rename = "SummonerInfoNotAvailable")]
    SummonerInfoNotAvailable,
    #[serde(rename = "LeaguesInfoNotAvailable")]
    LeaguesInfoNotAvailable,
    #[serde(rename = "InventoryChampsInfoNotAvailable")]
    InventoryChampsInfoNotAvailable,
    #[serde(rename = "InventoryQueuesInfoNotAvailable")]
    InventoryQueuesInfoNotAvailable,

}




