use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod builtin_api;
pub use self::builtin_api::{ BuiltinApi, BuiltinApiClient };
mod cookie_jar_api;
pub use self::cookie_jar_api::{ CookieJarApi, CookieJarApiClient };
mod core_api;
pub use self::core_api::{ CoreApi, CoreApiClient };
mod data_store_api;
pub use self::data_store_api::{ DataStoreApi, DataStoreApiClient };
mod default_api;
pub use self::default_api::{ DefaultApi, DefaultApiClient };
mod logging_api;
pub use self::logging_api::{ LoggingApi, LoggingApiClient };
mod performance_api;
pub use self::performance_api::{ PerformanceApi, PerformanceApiClient };
mod plugin_asset_serving_api;
pub use self::plugin_asset_serving_api::{ PluginAssetServingApi, PluginAssetServingApiClient };
mod plugin_client_config_api;
pub use self::plugin_client_config_api::{ PluginClientConfigApi, PluginClientConfigApiClient };
mod plugin_entitlements_api;
pub use self::plugin_entitlements_api::{ PluginEntitlementsApi, PluginEntitlementsApiClient };
mod plugin_gcloud_voice_chat_api;
pub use self::plugin_gcloud_voice_chat_api::{ PluginGcloudVoiceChatApi, PluginGcloudVoiceChatApiClient };
mod plugin_lol_account_verification_api;
pub use self::plugin_lol_account_verification_api::{ PluginLolAccountVerificationApi, PluginLolAccountVerificationApiClient };
mod plugin_lol_acs_api;
pub use self::plugin_lol_acs_api::{ PluginLolAcsApi, PluginLolAcsApiClient };
mod plugin_lol_active_boosts_api;
pub use self::plugin_lol_active_boosts_api::{ PluginLolActiveBoostsApi, PluginLolActiveBoostsApiClient };
mod plugin_lol_banners_api;
pub use self::plugin_lol_banners_api::{ PluginLolBannersApi, PluginLolBannersApiClient };
mod plugin_lol_career_stats_api;
pub use self::plugin_lol_career_stats_api::{ PluginLolCareerStatsApi, PluginLolCareerStatsApiClient };
mod plugin_lol_catalog_api;
pub use self::plugin_lol_catalog_api::{ PluginLolCatalogApi, PluginLolCatalogApiClient };
mod plugin_lol_champ_select_api;
pub use self::plugin_lol_champ_select_api::{ PluginLolChampSelectApi, PluginLolChampSelectApiClient };
mod plugin_lol_champ_select_legacy_api;
pub use self::plugin_lol_champ_select_legacy_api::{ PluginLolChampSelectLegacyApi, PluginLolChampSelectLegacyApiClient };
mod plugin_lol_champions_api;
pub use self::plugin_lol_champions_api::{ PluginLolChampionsApi, PluginLolChampionsApiClient };
mod plugin_lol_chat_api;
pub use self::plugin_lol_chat_api::{ PluginLolChatApi, PluginLolChatApiClient };
mod plugin_lol_clash_api;
pub use self::plugin_lol_clash_api::{ PluginLolClashApi, PluginLolClashApiClient };
mod plugin_lol_clubs_api;
pub use self::plugin_lol_clubs_api::{ PluginLolClubsApi, PluginLolClubsApiClient };
mod plugin_lol_clubs_public_api;
pub use self::plugin_lol_clubs_public_api::{ PluginLolClubsPublicApi, PluginLolClubsPublicApiClient };
mod plugin_lol_collections_api;
pub use self::plugin_lol_collections_api::{ PluginLolCollectionsApi, PluginLolCollectionsApiClient };
mod plugin_lol_content_targeting_api;
pub use self::plugin_lol_content_targeting_api::{ PluginLolContentTargetingApi, PluginLolContentTargetingApiClient };
mod plugin_lol_cosmetics_api;
pub use self::plugin_lol_cosmetics_api::{ PluginLolCosmeticsApi, PluginLolCosmeticsApiClient };
mod plugin_lol_email_verification_api;
pub use self::plugin_lol_email_verification_api::{ PluginLolEmailVerificationApi, PluginLolEmailVerificationApiClient };
mod plugin_lol_end_of_game_api;
pub use self::plugin_lol_end_of_game_api::{ PluginLolEndOfGameApi, PluginLolEndOfGameApiClient };
mod plugin_lol_esport_stream_notifications_api;
pub use self::plugin_lol_esport_stream_notifications_api::{ PluginLolEsportStreamNotificationsApi, PluginLolEsportStreamNotificationsApiClient };
mod plugin_lol_game_client_chat_api;
pub use self::plugin_lol_game_client_chat_api::{ PluginLolGameClientChatApi, PluginLolGameClientChatApiClient };
mod plugin_lol_game_queues_api;
pub use self::plugin_lol_game_queues_api::{ PluginLolGameQueuesApi, PluginLolGameQueuesApiClient };
mod plugin_lol_game_settings_api;
pub use self::plugin_lol_game_settings_api::{ PluginLolGameSettingsApi, PluginLolGameSettingsApiClient };
mod plugin_lol_gameflow_api;
pub use self::plugin_lol_gameflow_api::{ PluginLolGameflowApi, PluginLolGameflowApiClient };
mod plugin_lol_gamhs_api;
pub use self::plugin_lol_gamhs_api::{ PluginLolGamhsApi, PluginLolGamhsApiClient };
mod plugin_lol_geoinfo_api;
pub use self::plugin_lol_geoinfo_api::{ PluginLolGeoinfoApi, PluginLolGeoinfoApiClient };
mod plugin_lol_highlights_api;
pub use self::plugin_lol_highlights_api::{ PluginLolHighlightsApi, PluginLolHighlightsApiClient };
mod plugin_lol_honor_v2_api;
pub use self::plugin_lol_honor_v2_api::{ PluginLolHonorV2Api, PluginLolHonorV2ApiClient };
mod plugin_lol_hovercard_api;
pub use self::plugin_lol_hovercard_api::{ PluginLolHovercardApi, PluginLolHovercardApiClient };
mod plugin_lol_inventory_api;
pub use self::plugin_lol_inventory_api::{ PluginLolInventoryApi, PluginLolInventoryApiClient };
mod plugin_lol_item_sets_api;
pub use self::plugin_lol_item_sets_api::{ PluginLolItemSetsApi, PluginLolItemSetsApiClient };
mod plugin_lol_kickout_api;
pub use self::plugin_lol_kickout_api::{ PluginLolKickoutApi, PluginLolKickoutApiClient };
mod plugin_lol_kr_playtime_reminder_api;
pub use self::plugin_lol_kr_playtime_reminder_api::{ PluginLolKrPlaytimeReminderApi, PluginLolKrPlaytimeReminderApiClient };
mod plugin_lol_kr_shutdown_law_api;
pub use self::plugin_lol_kr_shutdown_law_api::{ PluginLolKrShutdownLawApi, PluginLolKrShutdownLawApiClient };
mod plugin_lol_league_session_api;
pub use self::plugin_lol_league_session_api::{ PluginLolLeagueSessionApi, PluginLolLeagueSessionApiClient };
mod plugin_lol_leaver_buster_api;
pub use self::plugin_lol_leaver_buster_api::{ PluginLolLeaverBusterApi, PluginLolLeaverBusterApiClient };
mod plugin_lol_license_agreement_api;
pub use self::plugin_lol_license_agreement_api::{ PluginLolLicenseAgreementApi, PluginLolLicenseAgreementApiClient };
mod plugin_lol_loadouts_api;
pub use self::plugin_lol_loadouts_api::{ PluginLolLoadoutsApi, PluginLolLoadoutsApiClient };
mod plugin_lol_lobby_api;
pub use self::plugin_lol_lobby_api::{ PluginLolLobbyApi, PluginLolLobbyApiClient };
mod plugin_lol_lobby_team_builder_api;
pub use self::plugin_lol_lobby_team_builder_api::{ PluginLolLobbyTeamBuilderApi, PluginLolLobbyTeamBuilderApiClient };
mod plugin_lol_login_api;
pub use self::plugin_lol_login_api::{ PluginLolLoginApi, PluginLolLoginApiClient };
mod plugin_lol_loot_api;
pub use self::plugin_lol_loot_api::{ PluginLolLootApi, PluginLolLootApiClient };
mod plugin_lol_loyalty_api;
pub use self::plugin_lol_loyalty_api::{ PluginLolLoyaltyApi, PluginLolLoyaltyApiClient };
mod plugin_lol_maps_api;
pub use self::plugin_lol_maps_api::{ PluginLolMapsApi, PluginLolMapsApiClient };
mod plugin_lol_match_history_api;
pub use self::plugin_lol_match_history_api::{ PluginLolMatchHistoryApi, PluginLolMatchHistoryApiClient };
mod plugin_lol_matchmaking_api;
pub use self::plugin_lol_matchmaking_api::{ PluginLolMatchmakingApi, PluginLolMatchmakingApiClient };
mod plugin_lol_missions_api;
pub use self::plugin_lol_missions_api::{ PluginLolMissionsApi, PluginLolMissionsApiClient };
mod plugin_lol_npe_rewards_api;
pub use self::plugin_lol_npe_rewards_api::{ PluginLolNpeRewardsApi, PluginLolNpeRewardsApiClient };
mod plugin_lol_npe_tutorial_path_api;
pub use self::plugin_lol_npe_tutorial_path_api::{ PluginLolNpeTutorialPathApi, PluginLolNpeTutorialPathApiClient };
mod plugin_lol_patch_api;
pub use self::plugin_lol_patch_api::{ PluginLolPatchApi, PluginLolPatchApiClient };
mod plugin_lol_perks_api;
pub use self::plugin_lol_perks_api::{ PluginLolPerksApi, PluginLolPerksApiClient };
mod plugin_lol_personalized_offers_api;
pub use self::plugin_lol_personalized_offers_api::{ PluginLolPersonalizedOffersApi, PluginLolPersonalizedOffersApiClient };
mod plugin_lol_pft_api;
pub use self::plugin_lol_pft_api::{ PluginLolPftApi, PluginLolPftApiClient };
mod plugin_lol_platform_config_api;
pub use self::plugin_lol_platform_config_api::{ PluginLolPlatformConfigApi, PluginLolPlatformConfigApiClient };
mod plugin_lol_player_behavior_api;
pub use self::plugin_lol_player_behavior_api::{ PluginLolPlayerBehaviorApi, PluginLolPlayerBehaviorApiClient };
mod plugin_lol_player_level_up_api;
pub use self::plugin_lol_player_level_up_api::{ PluginLolPlayerLevelUpApi, PluginLolPlayerLevelUpApiClient };
mod plugin_lol_player_messaging_api;
pub use self::plugin_lol_player_messaging_api::{ PluginLolPlayerMessagingApi, PluginLolPlayerMessagingApiClient };
mod plugin_lol_player_preferences_api;
pub use self::plugin_lol_player_preferences_api::{ PluginLolPlayerPreferencesApi, PluginLolPlayerPreferencesApiClient };
mod plugin_lol_pre_end_of_game_api;
pub use self::plugin_lol_pre_end_of_game_api::{ PluginLolPreEndOfGameApi, PluginLolPreEndOfGameApiClient };
mod plugin_lol_premade_voice_api;
pub use self::plugin_lol_premade_voice_api::{ PluginLolPremadeVoiceApi, PluginLolPremadeVoiceApiClient };
mod plugin_lol_purchase_widget_api;
pub use self::plugin_lol_purchase_widget_api::{ PluginLolPurchaseWidgetApi, PluginLolPurchaseWidgetApiClient };
mod plugin_lol_ranked_api;
pub use self::plugin_lol_ranked_api::{ PluginLolRankedApi, PluginLolRankedApiClient };
mod plugin_lol_recommendations_api;
pub use self::plugin_lol_recommendations_api::{ PluginLolRecommendationsApi, PluginLolRecommendationsApiClient };
mod plugin_lol_regalia_api;
pub use self::plugin_lol_regalia_api::{ PluginLolRegaliaApi, PluginLolRegaliaApiClient };
mod plugin_lol_replays_api;
pub use self::plugin_lol_replays_api::{ PluginLolReplaysApi, PluginLolReplaysApiClient };
mod plugin_lol_riot_messaging_service_api;
pub use self::plugin_lol_riot_messaging_service_api::{ PluginLolRiotMessagingServiceApi, PluginLolRiotMessagingServiceApiClient };
mod plugin_lol_rso_auth_api;
pub use self::plugin_lol_rso_auth_api::{ PluginLolRsoAuthApi, PluginLolRsoAuthApiClient };
mod plugin_lol_service_status_api;
pub use self::plugin_lol_service_status_api::{ PluginLolServiceStatusApi, PluginLolServiceStatusApiClient };
mod plugin_lol_settings_api;
pub use self::plugin_lol_settings_api::{ PluginLolSettingsApi, PluginLolSettingsApiClient };
mod plugin_lol_shutdown_api;
pub use self::plugin_lol_shutdown_api::{ PluginLolShutdownApi, PluginLolShutdownApiClient };
mod plugin_lol_simple_dialog_messages_api;
pub use self::plugin_lol_simple_dialog_messages_api::{ PluginLolSimpleDialogMessagesApi, PluginLolSimpleDialogMessagesApiClient };
mod plugin_lol_spectator_api;
pub use self::plugin_lol_spectator_api::{ PluginLolSpectatorApi, PluginLolSpectatorApiClient };
mod plugin_lol_statstones_api;
pub use self::plugin_lol_statstones_api::{ PluginLolStatstonesApi, PluginLolStatstonesApiClient };
mod plugin_lol_store_api;
pub use self::plugin_lol_store_api::{ PluginLolStoreApi, PluginLolStoreApiClient };
mod plugin_lol_suggested_players_api;
pub use self::plugin_lol_suggested_players_api::{ PluginLolSuggestedPlayersApi, PluginLolSuggestedPlayersApiClient };
mod plugin_lol_summoner_api;
pub use self::plugin_lol_summoner_api::{ PluginLolSummonerApi, PluginLolSummonerApiClient };
mod plugin_lol_tastes_api;
pub use self::plugin_lol_tastes_api::{ PluginLolTastesApi, PluginLolTastesApiClient };
mod plugin_lol_tencent_antiaddiction_api;
pub use self::plugin_lol_tencent_antiaddiction_api::{ PluginLolTencentAntiaddictionApi, PluginLolTencentAntiaddictionApiClient };
mod plugin_lol_trophies_api;
pub use self::plugin_lol_trophies_api::{ PluginLolTrophiesApi, PluginLolTrophiesApiClient };
mod plugin_lol_worlds_token_card_api;
pub use self::plugin_lol_worlds_token_card_api::{ PluginLolWorldsTokenCardApi, PluginLolWorldsTokenCardApiClient };
mod plugin_manager_api;
pub use self::plugin_manager_api::{ PluginManagerApi, PluginManagerApiClient };
mod plugin_manager_diagnostics_api;
pub use self::plugin_manager_diagnostics_api::{ PluginManagerDiagnosticsApi, PluginManagerDiagnosticsApiClient };
mod plugin_manager_info_api;
pub use self::plugin_manager_info_api::{ PluginManagerInfoApi, PluginManagerInfoApiClient };
mod plugin_patcher_api;
pub use self::plugin_patcher_api::{ PluginPatcherApi, PluginPatcherApiClient };
mod plugin_payments_api;
pub use self::plugin_payments_api::{ PluginPaymentsApi, PluginPaymentsApiClient };
mod plugin_player_notifications_api;
pub use self::plugin_player_notifications_api::{ PluginPlayerNotificationsApi, PluginPlayerNotificationsApiClient };
mod plugin_recofriender_api;
pub use self::plugin_recofriender_api::{ PluginRecofrienderApi, PluginRecofrienderApiClient };
mod plugin_riot_messaging_service_api;
pub use self::plugin_riot_messaging_service_api::{ PluginRiotMessagingServiceApi, PluginRiotMessagingServiceApiClient };
mod plugin_sanitizer_api;
pub use self::plugin_sanitizer_api::{ PluginSanitizerApi, PluginSanitizerApiClient };
mod plugin_voice_chat_api;
pub use self::plugin_voice_chat_api::{ PluginVoiceChatApi, PluginVoiceChatApiClient };
mod plugins_api;
pub use self::plugins_api::{ PluginsApi, PluginsApiClient };
mod process_control_api;
pub use self::process_control_api::{ ProcessControlApi, ProcessControlApiClient };
mod riotclient_api;
pub use self::riotclient_api::{ RiotclientApi, RiotclientApiClient };
mod system_api;
pub use self::system_api::{ SystemApi, SystemApiClient };
mod telemetry_api;
pub use self::telemetry_api::{ TelemetryApi, TelemetryApiClient };
mod tracing_api;
pub use self::tracing_api::{ TracingApi, TracingApiClient };

pub mod configuration;
pub mod client;
