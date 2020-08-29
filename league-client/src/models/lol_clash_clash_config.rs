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
pub struct LolClashClashConfig {
    #[serde(rename = "CheckPartiesRegistration", skip_serializing_if = "Option::is_none")]
    pub check_parties_registration: Option<bool>,
    #[serde(rename = "DisabledEvents", skip_serializing_if = "Option::is_none")]
    pub disabled_events: Option<Vec<String>>,
    #[serde(rename = "DisabledReason", skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    #[serde(rename = "EnabledState", skip_serializing_if = "Option::is_none")]
    pub enabled_state: Option<crate::models::LolClashClashState>,
    #[serde(rename = "EstimatedEnableTimeMillis", skip_serializing_if = "Option::is_none")]
    pub estimated_enable_time_millis: Option<i64>,
    #[serde(rename = "EventSendingEnabled", skip_serializing_if = "Option::is_none")]
    pub event_sending_enabled: Option<bool>,
    #[serde(rename = "HonorLevelRequired", skip_serializing_if = "Option::is_none")]
    pub honor_level_required: Option<i32>,
    #[serde(rename = "HonorRefreshRetrySeconds", skip_serializing_if = "Option::is_none")]
    pub honor_refresh_retry_seconds: Option<i32>,
    #[serde(rename = "IconConfig", skip_serializing_if = "Option::is_none")]
    pub icon_config: Option<String>,
    #[serde(rename = "IsPlaymodeRestrictionEnabled", skip_serializing_if = "Option::is_none")]
    pub is_playmode_restriction_enabled: Option<bool>,
    #[serde(rename = "MaxTimeBeforeLockinNotifySeconds", skip_serializing_if = "Option::is_none")]
    pub max_time_before_lockin_notify_seconds: Option<i32>,
    #[serde(rename = "MinClashNotificationsSummonerLevel", skip_serializing_if = "Option::is_none")]
    pub min_clash_notifications_summoner_level: Option<i32>,
    #[serde(rename = "MinClashSummonerLevel", skip_serializing_if = "Option::is_none")]
    pub min_clash_summoner_level: Option<i32>,
    #[serde(rename = "RewardGrantRetryIntervalSeconds", skip_serializing_if = "Option::is_none")]
    pub reward_grant_retry_interval_seconds: Option<i32>,
    #[serde(rename = "Visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<crate::models::LolClashClashVisibility>,
    #[serde(rename = "VoiceEobQuitDelaySeconds", skip_serializing_if = "Option::is_none")]
    pub voice_eob_quit_delay_seconds: Option<i32>,
    #[serde(rename = "VoiceNoDelayAutoStartSeconds", skip_serializing_if = "Option::is_none")]
    pub voice_no_delay_auto_start_seconds: Option<i32>,
    #[serde(rename = "VoiceRandomStartMaxSeconds", skip_serializing_if = "Option::is_none")]
    pub voice_random_start_max_seconds: Option<i32>,
    #[serde(rename = "VoiceRandomStartMinSeconds", skip_serializing_if = "Option::is_none")]
    pub voice_random_start_min_seconds: Option<i32>,
    #[serde(rename = "VoiceRetryCountLimit", skip_serializing_if = "Option::is_none")]
    pub voice_retry_count_limit: Option<i32>,
    #[serde(rename = "VoiceRetryDelaySeconds", skip_serializing_if = "Option::is_none")]
    pub voice_retry_delay_seconds: Option<i32>,
}

impl LolClashClashConfig {
    pub fn new() -> LolClashClashConfig {
        LolClashClashConfig {
            check_parties_registration: None,
            disabled_events: None,
            disabled_reason: None,
            enabled_state: None,
            estimated_enable_time_millis: None,
            event_sending_enabled: None,
            honor_level_required: None,
            honor_refresh_retry_seconds: None,
            icon_config: None,
            is_playmode_restriction_enabled: None,
            max_time_before_lockin_notify_seconds: None,
            min_clash_notifications_summoner_level: None,
            min_clash_summoner_level: None,
            reward_grant_retry_interval_seconds: None,
            visibility: None,
            voice_eob_quit_delay_seconds: None,
            voice_no_delay_auto_start_seconds: None,
            voice_random_start_max_seconds: None,
            voice_random_start_min_seconds: None,
            voice_retry_count_limit: None,
            voice_retry_delay_seconds: None,
        }
    }
}

