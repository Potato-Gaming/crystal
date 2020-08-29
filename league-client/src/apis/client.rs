use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    builtin_api: Box<dyn crate::apis::BuiltinApi>,
    cookie_jar_api: Box<dyn crate::apis::CookieJarApi>,
    core_api: Box<dyn crate::apis::CoreApi>,
    data_store_api: Box<dyn crate::apis::DataStoreApi>,
    default_api: Box<dyn crate::apis::DefaultApi>,
    logging_api: Box<dyn crate::apis::LoggingApi>,
    performance_api: Box<dyn crate::apis::PerformanceApi>,
    plugin_asset_serving_api: Box<dyn crate::apis::PluginAssetServingApi>,
    plugin_client_config_api: Box<dyn crate::apis::PluginClientConfigApi>,
    plugin_entitlements_api: Box<dyn crate::apis::PluginEntitlementsApi>,
    plugin_gcloud_voice_chat_api: Box<dyn crate::apis::PluginGcloudVoiceChatApi>,
    plugin_lol_account_verification_api: Box<dyn crate::apis::PluginLolAccountVerificationApi>,
    plugin_lol_acs_api: Box<dyn crate::apis::PluginLolAcsApi>,
    plugin_lol_active_boosts_api: Box<dyn crate::apis::PluginLolActiveBoostsApi>,
    plugin_lol_banners_api: Box<dyn crate::apis::PluginLolBannersApi>,
    plugin_lol_career_stats_api: Box<dyn crate::apis::PluginLolCareerStatsApi>,
    plugin_lol_catalog_api: Box<dyn crate::apis::PluginLolCatalogApi>,
    plugin_lol_champ_select_api: Box<dyn crate::apis::PluginLolChampSelectApi>,
    plugin_lol_champ_select_legacy_api: Box<dyn crate::apis::PluginLolChampSelectLegacyApi>,
    plugin_lol_champions_api: Box<dyn crate::apis::PluginLolChampionsApi>,
    plugin_lol_chat_api: Box<dyn crate::apis::PluginLolChatApi>,
    plugin_lol_clash_api: Box<dyn crate::apis::PluginLolClashApi>,
    plugin_lol_clubs_api: Box<dyn crate::apis::PluginLolClubsApi>,
    plugin_lol_clubs_public_api: Box<dyn crate::apis::PluginLolClubsPublicApi>,
    plugin_lol_collections_api: Box<dyn crate::apis::PluginLolCollectionsApi>,
    plugin_lol_content_targeting_api: Box<dyn crate::apis::PluginLolContentTargetingApi>,
    plugin_lol_cosmetics_api: Box<dyn crate::apis::PluginLolCosmeticsApi>,
    plugin_lol_email_verification_api: Box<dyn crate::apis::PluginLolEmailVerificationApi>,
    plugin_lol_end_of_game_api: Box<dyn crate::apis::PluginLolEndOfGameApi>,
    plugin_lol_esport_stream_notifications_api: Box<dyn crate::apis::PluginLolEsportStreamNotificationsApi>,
    plugin_lol_game_client_chat_api: Box<dyn crate::apis::PluginLolGameClientChatApi>,
    plugin_lol_game_queues_api: Box<dyn crate::apis::PluginLolGameQueuesApi>,
    plugin_lol_game_settings_api: Box<dyn crate::apis::PluginLolGameSettingsApi>,
    plugin_lol_gameflow_api: Box<dyn crate::apis::PluginLolGameflowApi>,
    plugin_lol_gamhs_api: Box<dyn crate::apis::PluginLolGamhsApi>,
    plugin_lol_geoinfo_api: Box<dyn crate::apis::PluginLolGeoinfoApi>,
    plugin_lol_highlights_api: Box<dyn crate::apis::PluginLolHighlightsApi>,
    plugin_lol_honor_v2_api: Box<dyn crate::apis::PluginLolHonorV2Api>,
    plugin_lol_hovercard_api: Box<dyn crate::apis::PluginLolHovercardApi>,
    plugin_lol_inventory_api: Box<dyn crate::apis::PluginLolInventoryApi>,
    plugin_lol_item_sets_api: Box<dyn crate::apis::PluginLolItemSetsApi>,
    plugin_lol_kickout_api: Box<dyn crate::apis::PluginLolKickoutApi>,
    plugin_lol_kr_playtime_reminder_api: Box<dyn crate::apis::PluginLolKrPlaytimeReminderApi>,
    plugin_lol_kr_shutdown_law_api: Box<dyn crate::apis::PluginLolKrShutdownLawApi>,
    plugin_lol_league_session_api: Box<dyn crate::apis::PluginLolLeagueSessionApi>,
    plugin_lol_leaver_buster_api: Box<dyn crate::apis::PluginLolLeaverBusterApi>,
    plugin_lol_license_agreement_api: Box<dyn crate::apis::PluginLolLicenseAgreementApi>,
    plugin_lol_loadouts_api: Box<dyn crate::apis::PluginLolLoadoutsApi>,
    plugin_lol_lobby_api: Box<dyn crate::apis::PluginLolLobbyApi>,
    plugin_lol_lobby_team_builder_api: Box<dyn crate::apis::PluginLolLobbyTeamBuilderApi>,
    plugin_lol_login_api: Box<dyn crate::apis::PluginLolLoginApi>,
    plugin_lol_loot_api: Box<dyn crate::apis::PluginLolLootApi>,
    plugin_lol_loyalty_api: Box<dyn crate::apis::PluginLolLoyaltyApi>,
    plugin_lol_maps_api: Box<dyn crate::apis::PluginLolMapsApi>,
    plugin_lol_match_history_api: Box<dyn crate::apis::PluginLolMatchHistoryApi>,
    plugin_lol_matchmaking_api: Box<dyn crate::apis::PluginLolMatchmakingApi>,
    plugin_lol_missions_api: Box<dyn crate::apis::PluginLolMissionsApi>,
    plugin_lol_npe_rewards_api: Box<dyn crate::apis::PluginLolNpeRewardsApi>,
    plugin_lol_npe_tutorial_path_api: Box<dyn crate::apis::PluginLolNpeTutorialPathApi>,
    plugin_lol_patch_api: Box<dyn crate::apis::PluginLolPatchApi>,
    plugin_lol_perks_api: Box<dyn crate::apis::PluginLolPerksApi>,
    plugin_lol_personalized_offers_api: Box<dyn crate::apis::PluginLolPersonalizedOffersApi>,
    plugin_lol_pft_api: Box<dyn crate::apis::PluginLolPftApi>,
    plugin_lol_platform_config_api: Box<dyn crate::apis::PluginLolPlatformConfigApi>,
    plugin_lol_player_behavior_api: Box<dyn crate::apis::PluginLolPlayerBehaviorApi>,
    plugin_lol_player_level_up_api: Box<dyn crate::apis::PluginLolPlayerLevelUpApi>,
    plugin_lol_player_messaging_api: Box<dyn crate::apis::PluginLolPlayerMessagingApi>,
    plugin_lol_player_preferences_api: Box<dyn crate::apis::PluginLolPlayerPreferencesApi>,
    plugin_lol_pre_end_of_game_api: Box<dyn crate::apis::PluginLolPreEndOfGameApi>,
    plugin_lol_premade_voice_api: Box<dyn crate::apis::PluginLolPremadeVoiceApi>,
    plugin_lol_purchase_widget_api: Box<dyn crate::apis::PluginLolPurchaseWidgetApi>,
    plugin_lol_ranked_api: Box<dyn crate::apis::PluginLolRankedApi>,
    plugin_lol_recommendations_api: Box<dyn crate::apis::PluginLolRecommendationsApi>,
    plugin_lol_regalia_api: Box<dyn crate::apis::PluginLolRegaliaApi>,
    plugin_lol_replays_api: Box<dyn crate::apis::PluginLolReplaysApi>,
    plugin_lol_riot_messaging_service_api: Box<dyn crate::apis::PluginLolRiotMessagingServiceApi>,
    plugin_lol_rso_auth_api: Box<dyn crate::apis::PluginLolRsoAuthApi>,
    plugin_lol_service_status_api: Box<dyn crate::apis::PluginLolServiceStatusApi>,
    plugin_lol_settings_api: Box<dyn crate::apis::PluginLolSettingsApi>,
    plugin_lol_shutdown_api: Box<dyn crate::apis::PluginLolShutdownApi>,
    plugin_lol_simple_dialog_messages_api: Box<dyn crate::apis::PluginLolSimpleDialogMessagesApi>,
    plugin_lol_spectator_api: Box<dyn crate::apis::PluginLolSpectatorApi>,
    plugin_lol_statstones_api: Box<dyn crate::apis::PluginLolStatstonesApi>,
    plugin_lol_store_api: Box<dyn crate::apis::PluginLolStoreApi>,
    plugin_lol_suggested_players_api: Box<dyn crate::apis::PluginLolSuggestedPlayersApi>,
    plugin_lol_summoner_api: Box<dyn crate::apis::PluginLolSummonerApi>,
    plugin_lol_tastes_api: Box<dyn crate::apis::PluginLolTastesApi>,
    plugin_lol_tencent_antiaddiction_api: Box<dyn crate::apis::PluginLolTencentAntiaddictionApi>,
    plugin_lol_trophies_api: Box<dyn crate::apis::PluginLolTrophiesApi>,
    plugin_lol_worlds_token_card_api: Box<dyn crate::apis::PluginLolWorldsTokenCardApi>,
    plugin_manager_api: Box<dyn crate::apis::PluginManagerApi>,
    plugin_manager_diagnostics_api: Box<dyn crate::apis::PluginManagerDiagnosticsApi>,
    plugin_manager_info_api: Box<dyn crate::apis::PluginManagerInfoApi>,
    plugin_patcher_api: Box<dyn crate::apis::PluginPatcherApi>,
    plugin_payments_api: Box<dyn crate::apis::PluginPaymentsApi>,
    plugin_player_notifications_api: Box<dyn crate::apis::PluginPlayerNotificationsApi>,
    plugin_recofriender_api: Box<dyn crate::apis::PluginRecofrienderApi>,
    plugin_riot_messaging_service_api: Box<dyn crate::apis::PluginRiotMessagingServiceApi>,
    plugin_sanitizer_api: Box<dyn crate::apis::PluginSanitizerApi>,
    plugin_voice_chat_api: Box<dyn crate::apis::PluginVoiceChatApi>,
    plugins_api: Box<dyn crate::apis::PluginsApi>,
    process_control_api: Box<dyn crate::apis::ProcessControlApi>,
    riotclient_api: Box<dyn crate::apis::RiotclientApi>,
    system_api: Box<dyn crate::apis::SystemApi>,
    telemetry_api: Box<dyn crate::apis::TelemetryApi>,
    tracing_api: Box<dyn crate::apis::TracingApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            builtin_api: Box::new(crate::apis::BuiltinApiClient::new(rc.clone())),
            cookie_jar_api: Box::new(crate::apis::CookieJarApiClient::new(rc.clone())),
            core_api: Box::new(crate::apis::CoreApiClient::new(rc.clone())),
            data_store_api: Box::new(crate::apis::DataStoreApiClient::new(rc.clone())),
            default_api: Box::new(crate::apis::DefaultApiClient::new(rc.clone())),
            logging_api: Box::new(crate::apis::LoggingApiClient::new(rc.clone())),
            performance_api: Box::new(crate::apis::PerformanceApiClient::new(rc.clone())),
            plugin_asset_serving_api: Box::new(crate::apis::PluginAssetServingApiClient::new(rc.clone())),
            plugin_client_config_api: Box::new(crate::apis::PluginClientConfigApiClient::new(rc.clone())),
            plugin_entitlements_api: Box::new(crate::apis::PluginEntitlementsApiClient::new(rc.clone())),
            plugin_gcloud_voice_chat_api: Box::new(crate::apis::PluginGcloudVoiceChatApiClient::new(rc.clone())),
            plugin_lol_account_verification_api: Box::new(crate::apis::PluginLolAccountVerificationApiClient::new(rc.clone())),
            plugin_lol_acs_api: Box::new(crate::apis::PluginLolAcsApiClient::new(rc.clone())),
            plugin_lol_active_boosts_api: Box::new(crate::apis::PluginLolActiveBoostsApiClient::new(rc.clone())),
            plugin_lol_banners_api: Box::new(crate::apis::PluginLolBannersApiClient::new(rc.clone())),
            plugin_lol_career_stats_api: Box::new(crate::apis::PluginLolCareerStatsApiClient::new(rc.clone())),
            plugin_lol_catalog_api: Box::new(crate::apis::PluginLolCatalogApiClient::new(rc.clone())),
            plugin_lol_champ_select_api: Box::new(crate::apis::PluginLolChampSelectApiClient::new(rc.clone())),
            plugin_lol_champ_select_legacy_api: Box::new(crate::apis::PluginLolChampSelectLegacyApiClient::new(rc.clone())),
            plugin_lol_champions_api: Box::new(crate::apis::PluginLolChampionsApiClient::new(rc.clone())),
            plugin_lol_chat_api: Box::new(crate::apis::PluginLolChatApiClient::new(rc.clone())),
            plugin_lol_clash_api: Box::new(crate::apis::PluginLolClashApiClient::new(rc.clone())),
            plugin_lol_clubs_api: Box::new(crate::apis::PluginLolClubsApiClient::new(rc.clone())),
            plugin_lol_clubs_public_api: Box::new(crate::apis::PluginLolClubsPublicApiClient::new(rc.clone())),
            plugin_lol_collections_api: Box::new(crate::apis::PluginLolCollectionsApiClient::new(rc.clone())),
            plugin_lol_content_targeting_api: Box::new(crate::apis::PluginLolContentTargetingApiClient::new(rc.clone())),
            plugin_lol_cosmetics_api: Box::new(crate::apis::PluginLolCosmeticsApiClient::new(rc.clone())),
            plugin_lol_email_verification_api: Box::new(crate::apis::PluginLolEmailVerificationApiClient::new(rc.clone())),
            plugin_lol_end_of_game_api: Box::new(crate::apis::PluginLolEndOfGameApiClient::new(rc.clone())),
            plugin_lol_esport_stream_notifications_api: Box::new(crate::apis::PluginLolEsportStreamNotificationsApiClient::new(rc.clone())),
            plugin_lol_game_client_chat_api: Box::new(crate::apis::PluginLolGameClientChatApiClient::new(rc.clone())),
            plugin_lol_game_queues_api: Box::new(crate::apis::PluginLolGameQueuesApiClient::new(rc.clone())),
            plugin_lol_game_settings_api: Box::new(crate::apis::PluginLolGameSettingsApiClient::new(rc.clone())),
            plugin_lol_gameflow_api: Box::new(crate::apis::PluginLolGameflowApiClient::new(rc.clone())),
            plugin_lol_gamhs_api: Box::new(crate::apis::PluginLolGamhsApiClient::new(rc.clone())),
            plugin_lol_geoinfo_api: Box::new(crate::apis::PluginLolGeoinfoApiClient::new(rc.clone())),
            plugin_lol_highlights_api: Box::new(crate::apis::PluginLolHighlightsApiClient::new(rc.clone())),
            plugin_lol_honor_v2_api: Box::new(crate::apis::PluginLolHonorV2ApiClient::new(rc.clone())),
            plugin_lol_hovercard_api: Box::new(crate::apis::PluginLolHovercardApiClient::new(rc.clone())),
            plugin_lol_inventory_api: Box::new(crate::apis::PluginLolInventoryApiClient::new(rc.clone())),
            plugin_lol_item_sets_api: Box::new(crate::apis::PluginLolItemSetsApiClient::new(rc.clone())),
            plugin_lol_kickout_api: Box::new(crate::apis::PluginLolKickoutApiClient::new(rc.clone())),
            plugin_lol_kr_playtime_reminder_api: Box::new(crate::apis::PluginLolKrPlaytimeReminderApiClient::new(rc.clone())),
            plugin_lol_kr_shutdown_law_api: Box::new(crate::apis::PluginLolKrShutdownLawApiClient::new(rc.clone())),
            plugin_lol_league_session_api: Box::new(crate::apis::PluginLolLeagueSessionApiClient::new(rc.clone())),
            plugin_lol_leaver_buster_api: Box::new(crate::apis::PluginLolLeaverBusterApiClient::new(rc.clone())),
            plugin_lol_license_agreement_api: Box::new(crate::apis::PluginLolLicenseAgreementApiClient::new(rc.clone())),
            plugin_lol_loadouts_api: Box::new(crate::apis::PluginLolLoadoutsApiClient::new(rc.clone())),
            plugin_lol_lobby_api: Box::new(crate::apis::PluginLolLobbyApiClient::new(rc.clone())),
            plugin_lol_lobby_team_builder_api: Box::new(crate::apis::PluginLolLobbyTeamBuilderApiClient::new(rc.clone())),
            plugin_lol_login_api: Box::new(crate::apis::PluginLolLoginApiClient::new(rc.clone())),
            plugin_lol_loot_api: Box::new(crate::apis::PluginLolLootApiClient::new(rc.clone())),
            plugin_lol_loyalty_api: Box::new(crate::apis::PluginLolLoyaltyApiClient::new(rc.clone())),
            plugin_lol_maps_api: Box::new(crate::apis::PluginLolMapsApiClient::new(rc.clone())),
            plugin_lol_match_history_api: Box::new(crate::apis::PluginLolMatchHistoryApiClient::new(rc.clone())),
            plugin_lol_matchmaking_api: Box::new(crate::apis::PluginLolMatchmakingApiClient::new(rc.clone())),
            plugin_lol_missions_api: Box::new(crate::apis::PluginLolMissionsApiClient::new(rc.clone())),
            plugin_lol_npe_rewards_api: Box::new(crate::apis::PluginLolNpeRewardsApiClient::new(rc.clone())),
            plugin_lol_npe_tutorial_path_api: Box::new(crate::apis::PluginLolNpeTutorialPathApiClient::new(rc.clone())),
            plugin_lol_patch_api: Box::new(crate::apis::PluginLolPatchApiClient::new(rc.clone())),
            plugin_lol_perks_api: Box::new(crate::apis::PluginLolPerksApiClient::new(rc.clone())),
            plugin_lol_personalized_offers_api: Box::new(crate::apis::PluginLolPersonalizedOffersApiClient::new(rc.clone())),
            plugin_lol_pft_api: Box::new(crate::apis::PluginLolPftApiClient::new(rc.clone())),
            plugin_lol_platform_config_api: Box::new(crate::apis::PluginLolPlatformConfigApiClient::new(rc.clone())),
            plugin_lol_player_behavior_api: Box::new(crate::apis::PluginLolPlayerBehaviorApiClient::new(rc.clone())),
            plugin_lol_player_level_up_api: Box::new(crate::apis::PluginLolPlayerLevelUpApiClient::new(rc.clone())),
            plugin_lol_player_messaging_api: Box::new(crate::apis::PluginLolPlayerMessagingApiClient::new(rc.clone())),
            plugin_lol_player_preferences_api: Box::new(crate::apis::PluginLolPlayerPreferencesApiClient::new(rc.clone())),
            plugin_lol_pre_end_of_game_api: Box::new(crate::apis::PluginLolPreEndOfGameApiClient::new(rc.clone())),
            plugin_lol_premade_voice_api: Box::new(crate::apis::PluginLolPremadeVoiceApiClient::new(rc.clone())),
            plugin_lol_purchase_widget_api: Box::new(crate::apis::PluginLolPurchaseWidgetApiClient::new(rc.clone())),
            plugin_lol_ranked_api: Box::new(crate::apis::PluginLolRankedApiClient::new(rc.clone())),
            plugin_lol_recommendations_api: Box::new(crate::apis::PluginLolRecommendationsApiClient::new(rc.clone())),
            plugin_lol_regalia_api: Box::new(crate::apis::PluginLolRegaliaApiClient::new(rc.clone())),
            plugin_lol_replays_api: Box::new(crate::apis::PluginLolReplaysApiClient::new(rc.clone())),
            plugin_lol_riot_messaging_service_api: Box::new(crate::apis::PluginLolRiotMessagingServiceApiClient::new(rc.clone())),
            plugin_lol_rso_auth_api: Box::new(crate::apis::PluginLolRsoAuthApiClient::new(rc.clone())),
            plugin_lol_service_status_api: Box::new(crate::apis::PluginLolServiceStatusApiClient::new(rc.clone())),
            plugin_lol_settings_api: Box::new(crate::apis::PluginLolSettingsApiClient::new(rc.clone())),
            plugin_lol_shutdown_api: Box::new(crate::apis::PluginLolShutdownApiClient::new(rc.clone())),
            plugin_lol_simple_dialog_messages_api: Box::new(crate::apis::PluginLolSimpleDialogMessagesApiClient::new(rc.clone())),
            plugin_lol_spectator_api: Box::new(crate::apis::PluginLolSpectatorApiClient::new(rc.clone())),
            plugin_lol_statstones_api: Box::new(crate::apis::PluginLolStatstonesApiClient::new(rc.clone())),
            plugin_lol_store_api: Box::new(crate::apis::PluginLolStoreApiClient::new(rc.clone())),
            plugin_lol_suggested_players_api: Box::new(crate::apis::PluginLolSuggestedPlayersApiClient::new(rc.clone())),
            plugin_lol_summoner_api: Box::new(crate::apis::PluginLolSummonerApiClient::new(rc.clone())),
            plugin_lol_tastes_api: Box::new(crate::apis::PluginLolTastesApiClient::new(rc.clone())),
            plugin_lol_tencent_antiaddiction_api: Box::new(crate::apis::PluginLolTencentAntiaddictionApiClient::new(rc.clone())),
            plugin_lol_trophies_api: Box::new(crate::apis::PluginLolTrophiesApiClient::new(rc.clone())),
            plugin_lol_worlds_token_card_api: Box::new(crate::apis::PluginLolWorldsTokenCardApiClient::new(rc.clone())),
            plugin_manager_api: Box::new(crate::apis::PluginManagerApiClient::new(rc.clone())),
            plugin_manager_diagnostics_api: Box::new(crate::apis::PluginManagerDiagnosticsApiClient::new(rc.clone())),
            plugin_manager_info_api: Box::new(crate::apis::PluginManagerInfoApiClient::new(rc.clone())),
            plugin_patcher_api: Box::new(crate::apis::PluginPatcherApiClient::new(rc.clone())),
            plugin_payments_api: Box::new(crate::apis::PluginPaymentsApiClient::new(rc.clone())),
            plugin_player_notifications_api: Box::new(crate::apis::PluginPlayerNotificationsApiClient::new(rc.clone())),
            plugin_recofriender_api: Box::new(crate::apis::PluginRecofrienderApiClient::new(rc.clone())),
            plugin_riot_messaging_service_api: Box::new(crate::apis::PluginRiotMessagingServiceApiClient::new(rc.clone())),
            plugin_sanitizer_api: Box::new(crate::apis::PluginSanitizerApiClient::new(rc.clone())),
            plugin_voice_chat_api: Box::new(crate::apis::PluginVoiceChatApiClient::new(rc.clone())),
            plugins_api: Box::new(crate::apis::PluginsApiClient::new(rc.clone())),
            process_control_api: Box::new(crate::apis::ProcessControlApiClient::new(rc.clone())),
            riotclient_api: Box::new(crate::apis::RiotclientApiClient::new(rc.clone())),
            system_api: Box::new(crate::apis::SystemApiClient::new(rc.clone())),
            telemetry_api: Box::new(crate::apis::TelemetryApiClient::new(rc.clone())),
            tracing_api: Box::new(crate::apis::TracingApiClient::new(rc.clone())),
        }
    }

    pub fn builtin_api(&self) -> &dyn crate::apis::BuiltinApi{
        self.builtin_api.as_ref()
    }

    pub fn cookie_jar_api(&self) -> &dyn crate::apis::CookieJarApi{
        self.cookie_jar_api.as_ref()
    }

    pub fn core_api(&self) -> &dyn crate::apis::CoreApi{
        self.core_api.as_ref()
    }

    pub fn data_store_api(&self) -> &dyn crate::apis::DataStoreApi{
        self.data_store_api.as_ref()
    }

    pub fn default_api(&self) -> &dyn crate::apis::DefaultApi{
        self.default_api.as_ref()
    }

    pub fn logging_api(&self) -> &dyn crate::apis::LoggingApi{
        self.logging_api.as_ref()
    }

    pub fn performance_api(&self) -> &dyn crate::apis::PerformanceApi{
        self.performance_api.as_ref()
    }

    pub fn plugin_asset_serving_api(&self) -> &dyn crate::apis::PluginAssetServingApi{
        self.plugin_asset_serving_api.as_ref()
    }

    pub fn plugin_client_config_api(&self) -> &dyn crate::apis::PluginClientConfigApi{
        self.plugin_client_config_api.as_ref()
    }

    pub fn plugin_entitlements_api(&self) -> &dyn crate::apis::PluginEntitlementsApi{
        self.plugin_entitlements_api.as_ref()
    }

    pub fn plugin_gcloud_voice_chat_api(&self) -> &dyn crate::apis::PluginGcloudVoiceChatApi{
        self.plugin_gcloud_voice_chat_api.as_ref()
    }

    pub fn plugin_lol_account_verification_api(&self) -> &dyn crate::apis::PluginLolAccountVerificationApi{
        self.plugin_lol_account_verification_api.as_ref()
    }

    pub fn plugin_lol_acs_api(&self) -> &dyn crate::apis::PluginLolAcsApi{
        self.plugin_lol_acs_api.as_ref()
    }

    pub fn plugin_lol_active_boosts_api(&self) -> &dyn crate::apis::PluginLolActiveBoostsApi{
        self.plugin_lol_active_boosts_api.as_ref()
    }

    pub fn plugin_lol_banners_api(&self) -> &dyn crate::apis::PluginLolBannersApi{
        self.plugin_lol_banners_api.as_ref()
    }

    pub fn plugin_lol_career_stats_api(&self) -> &dyn crate::apis::PluginLolCareerStatsApi{
        self.plugin_lol_career_stats_api.as_ref()
    }

    pub fn plugin_lol_catalog_api(&self) -> &dyn crate::apis::PluginLolCatalogApi{
        self.plugin_lol_catalog_api.as_ref()
    }

    pub fn plugin_lol_champ_select_api(&self) -> &dyn crate::apis::PluginLolChampSelectApi{
        self.plugin_lol_champ_select_api.as_ref()
    }

    pub fn plugin_lol_champ_select_legacy_api(&self) -> &dyn crate::apis::PluginLolChampSelectLegacyApi{
        self.plugin_lol_champ_select_legacy_api.as_ref()
    }

    pub fn plugin_lol_champions_api(&self) -> &dyn crate::apis::PluginLolChampionsApi{
        self.plugin_lol_champions_api.as_ref()
    }

    pub fn plugin_lol_chat_api(&self) -> &dyn crate::apis::PluginLolChatApi{
        self.plugin_lol_chat_api.as_ref()
    }

    pub fn plugin_lol_clash_api(&self) -> &dyn crate::apis::PluginLolClashApi{
        self.plugin_lol_clash_api.as_ref()
    }

    pub fn plugin_lol_clubs_api(&self) -> &dyn crate::apis::PluginLolClubsApi{
        self.plugin_lol_clubs_api.as_ref()
    }

    pub fn plugin_lol_clubs_public_api(&self) -> &dyn crate::apis::PluginLolClubsPublicApi{
        self.plugin_lol_clubs_public_api.as_ref()
    }

    pub fn plugin_lol_collections_api(&self) -> &dyn crate::apis::PluginLolCollectionsApi{
        self.plugin_lol_collections_api.as_ref()
    }

    pub fn plugin_lol_content_targeting_api(&self) -> &dyn crate::apis::PluginLolContentTargetingApi{
        self.plugin_lol_content_targeting_api.as_ref()
    }

    pub fn plugin_lol_cosmetics_api(&self) -> &dyn crate::apis::PluginLolCosmeticsApi{
        self.plugin_lol_cosmetics_api.as_ref()
    }

    pub fn plugin_lol_email_verification_api(&self) -> &dyn crate::apis::PluginLolEmailVerificationApi{
        self.plugin_lol_email_verification_api.as_ref()
    }

    pub fn plugin_lol_end_of_game_api(&self) -> &dyn crate::apis::PluginLolEndOfGameApi{
        self.plugin_lol_end_of_game_api.as_ref()
    }

    pub fn plugin_lol_esport_stream_notifications_api(&self) -> &dyn crate::apis::PluginLolEsportStreamNotificationsApi{
        self.plugin_lol_esport_stream_notifications_api.as_ref()
    }

    pub fn plugin_lol_game_client_chat_api(&self) -> &dyn crate::apis::PluginLolGameClientChatApi{
        self.plugin_lol_game_client_chat_api.as_ref()
    }

    pub fn plugin_lol_game_queues_api(&self) -> &dyn crate::apis::PluginLolGameQueuesApi{
        self.plugin_lol_game_queues_api.as_ref()
    }

    pub fn plugin_lol_game_settings_api(&self) -> &dyn crate::apis::PluginLolGameSettingsApi{
        self.plugin_lol_game_settings_api.as_ref()
    }

    pub fn plugin_lol_gameflow_api(&self) -> &dyn crate::apis::PluginLolGameflowApi{
        self.plugin_lol_gameflow_api.as_ref()
    }

    pub fn plugin_lol_gamhs_api(&self) -> &dyn crate::apis::PluginLolGamhsApi{
        self.plugin_lol_gamhs_api.as_ref()
    }

    pub fn plugin_lol_geoinfo_api(&self) -> &dyn crate::apis::PluginLolGeoinfoApi{
        self.plugin_lol_geoinfo_api.as_ref()
    }

    pub fn plugin_lol_highlights_api(&self) -> &dyn crate::apis::PluginLolHighlightsApi{
        self.plugin_lol_highlights_api.as_ref()
    }

    pub fn plugin_lol_honor_v2_api(&self) -> &dyn crate::apis::PluginLolHonorV2Api{
        self.plugin_lol_honor_v2_api.as_ref()
    }

    pub fn plugin_lol_hovercard_api(&self) -> &dyn crate::apis::PluginLolHovercardApi{
        self.plugin_lol_hovercard_api.as_ref()
    }

    pub fn plugin_lol_inventory_api(&self) -> &dyn crate::apis::PluginLolInventoryApi{
        self.plugin_lol_inventory_api.as_ref()
    }

    pub fn plugin_lol_item_sets_api(&self) -> &dyn crate::apis::PluginLolItemSetsApi{
        self.plugin_lol_item_sets_api.as_ref()
    }

    pub fn plugin_lol_kickout_api(&self) -> &dyn crate::apis::PluginLolKickoutApi{
        self.plugin_lol_kickout_api.as_ref()
    }

    pub fn plugin_lol_kr_playtime_reminder_api(&self) -> &dyn crate::apis::PluginLolKrPlaytimeReminderApi{
        self.plugin_lol_kr_playtime_reminder_api.as_ref()
    }

    pub fn plugin_lol_kr_shutdown_law_api(&self) -> &dyn crate::apis::PluginLolKrShutdownLawApi{
        self.plugin_lol_kr_shutdown_law_api.as_ref()
    }

    pub fn plugin_lol_league_session_api(&self) -> &dyn crate::apis::PluginLolLeagueSessionApi{
        self.plugin_lol_league_session_api.as_ref()
    }

    pub fn plugin_lol_leaver_buster_api(&self) -> &dyn crate::apis::PluginLolLeaverBusterApi{
        self.plugin_lol_leaver_buster_api.as_ref()
    }

    pub fn plugin_lol_license_agreement_api(&self) -> &dyn crate::apis::PluginLolLicenseAgreementApi{
        self.plugin_lol_license_agreement_api.as_ref()
    }

    pub fn plugin_lol_loadouts_api(&self) -> &dyn crate::apis::PluginLolLoadoutsApi{
        self.plugin_lol_loadouts_api.as_ref()
    }

    pub fn plugin_lol_lobby_api(&self) -> &dyn crate::apis::PluginLolLobbyApi{
        self.plugin_lol_lobby_api.as_ref()
    }

    pub fn plugin_lol_lobby_team_builder_api(&self) -> &dyn crate::apis::PluginLolLobbyTeamBuilderApi{
        self.plugin_lol_lobby_team_builder_api.as_ref()
    }

    pub fn plugin_lol_login_api(&self) -> &dyn crate::apis::PluginLolLoginApi{
        self.plugin_lol_login_api.as_ref()
    }

    pub fn plugin_lol_loot_api(&self) -> &dyn crate::apis::PluginLolLootApi{
        self.plugin_lol_loot_api.as_ref()
    }

    pub fn plugin_lol_loyalty_api(&self) -> &dyn crate::apis::PluginLolLoyaltyApi{
        self.plugin_lol_loyalty_api.as_ref()
    }

    pub fn plugin_lol_maps_api(&self) -> &dyn crate::apis::PluginLolMapsApi{
        self.plugin_lol_maps_api.as_ref()
    }

    pub fn plugin_lol_match_history_api(&self) -> &dyn crate::apis::PluginLolMatchHistoryApi{
        self.plugin_lol_match_history_api.as_ref()
    }

    pub fn plugin_lol_matchmaking_api(&self) -> &dyn crate::apis::PluginLolMatchmakingApi{
        self.plugin_lol_matchmaking_api.as_ref()
    }

    pub fn plugin_lol_missions_api(&self) -> &dyn crate::apis::PluginLolMissionsApi{
        self.plugin_lol_missions_api.as_ref()
    }

    pub fn plugin_lol_npe_rewards_api(&self) -> &dyn crate::apis::PluginLolNpeRewardsApi{
        self.plugin_lol_npe_rewards_api.as_ref()
    }

    pub fn plugin_lol_npe_tutorial_path_api(&self) -> &dyn crate::apis::PluginLolNpeTutorialPathApi{
        self.plugin_lol_npe_tutorial_path_api.as_ref()
    }

    pub fn plugin_lol_patch_api(&self) -> &dyn crate::apis::PluginLolPatchApi{
        self.plugin_lol_patch_api.as_ref()
    }

    pub fn plugin_lol_perks_api(&self) -> &dyn crate::apis::PluginLolPerksApi{
        self.plugin_lol_perks_api.as_ref()
    }

    pub fn plugin_lol_personalized_offers_api(&self) -> &dyn crate::apis::PluginLolPersonalizedOffersApi{
        self.plugin_lol_personalized_offers_api.as_ref()
    }

    pub fn plugin_lol_pft_api(&self) -> &dyn crate::apis::PluginLolPftApi{
        self.plugin_lol_pft_api.as_ref()
    }

    pub fn plugin_lol_platform_config_api(&self) -> &dyn crate::apis::PluginLolPlatformConfigApi{
        self.plugin_lol_platform_config_api.as_ref()
    }

    pub fn plugin_lol_player_behavior_api(&self) -> &dyn crate::apis::PluginLolPlayerBehaviorApi{
        self.plugin_lol_player_behavior_api.as_ref()
    }

    pub fn plugin_lol_player_level_up_api(&self) -> &dyn crate::apis::PluginLolPlayerLevelUpApi{
        self.plugin_lol_player_level_up_api.as_ref()
    }

    pub fn plugin_lol_player_messaging_api(&self) -> &dyn crate::apis::PluginLolPlayerMessagingApi{
        self.plugin_lol_player_messaging_api.as_ref()
    }

    pub fn plugin_lol_player_preferences_api(&self) -> &dyn crate::apis::PluginLolPlayerPreferencesApi{
        self.plugin_lol_player_preferences_api.as_ref()
    }

    pub fn plugin_lol_pre_end_of_game_api(&self) -> &dyn crate::apis::PluginLolPreEndOfGameApi{
        self.plugin_lol_pre_end_of_game_api.as_ref()
    }

    pub fn plugin_lol_premade_voice_api(&self) -> &dyn crate::apis::PluginLolPremadeVoiceApi{
        self.plugin_lol_premade_voice_api.as_ref()
    }

    pub fn plugin_lol_purchase_widget_api(&self) -> &dyn crate::apis::PluginLolPurchaseWidgetApi{
        self.plugin_lol_purchase_widget_api.as_ref()
    }

    pub fn plugin_lol_ranked_api(&self) -> &dyn crate::apis::PluginLolRankedApi{
        self.plugin_lol_ranked_api.as_ref()
    }

    pub fn plugin_lol_recommendations_api(&self) -> &dyn crate::apis::PluginLolRecommendationsApi{
        self.plugin_lol_recommendations_api.as_ref()
    }

    pub fn plugin_lol_regalia_api(&self) -> &dyn crate::apis::PluginLolRegaliaApi{
        self.plugin_lol_regalia_api.as_ref()
    }

    pub fn plugin_lol_replays_api(&self) -> &dyn crate::apis::PluginLolReplaysApi{
        self.plugin_lol_replays_api.as_ref()
    }

    pub fn plugin_lol_riot_messaging_service_api(&self) -> &dyn crate::apis::PluginLolRiotMessagingServiceApi{
        self.plugin_lol_riot_messaging_service_api.as_ref()
    }

    pub fn plugin_lol_rso_auth_api(&self) -> &dyn crate::apis::PluginLolRsoAuthApi{
        self.plugin_lol_rso_auth_api.as_ref()
    }

    pub fn plugin_lol_service_status_api(&self) -> &dyn crate::apis::PluginLolServiceStatusApi{
        self.plugin_lol_service_status_api.as_ref()
    }

    pub fn plugin_lol_settings_api(&self) -> &dyn crate::apis::PluginLolSettingsApi{
        self.plugin_lol_settings_api.as_ref()
    }

    pub fn plugin_lol_shutdown_api(&self) -> &dyn crate::apis::PluginLolShutdownApi{
        self.plugin_lol_shutdown_api.as_ref()
    }

    pub fn plugin_lol_simple_dialog_messages_api(&self) -> &dyn crate::apis::PluginLolSimpleDialogMessagesApi{
        self.plugin_lol_simple_dialog_messages_api.as_ref()
    }

    pub fn plugin_lol_spectator_api(&self) -> &dyn crate::apis::PluginLolSpectatorApi{
        self.plugin_lol_spectator_api.as_ref()
    }

    pub fn plugin_lol_statstones_api(&self) -> &dyn crate::apis::PluginLolStatstonesApi{
        self.plugin_lol_statstones_api.as_ref()
    }

    pub fn plugin_lol_store_api(&self) -> &dyn crate::apis::PluginLolStoreApi{
        self.plugin_lol_store_api.as_ref()
    }

    pub fn plugin_lol_suggested_players_api(&self) -> &dyn crate::apis::PluginLolSuggestedPlayersApi{
        self.plugin_lol_suggested_players_api.as_ref()
    }

    pub fn plugin_lol_summoner_api(&self) -> &dyn crate::apis::PluginLolSummonerApi{
        self.plugin_lol_summoner_api.as_ref()
    }

    pub fn plugin_lol_tastes_api(&self) -> &dyn crate::apis::PluginLolTastesApi{
        self.plugin_lol_tastes_api.as_ref()
    }

    pub fn plugin_lol_tencent_antiaddiction_api(&self) -> &dyn crate::apis::PluginLolTencentAntiaddictionApi{
        self.plugin_lol_tencent_antiaddiction_api.as_ref()
    }

    pub fn plugin_lol_trophies_api(&self) -> &dyn crate::apis::PluginLolTrophiesApi{
        self.plugin_lol_trophies_api.as_ref()
    }

    pub fn plugin_lol_worlds_token_card_api(&self) -> &dyn crate::apis::PluginLolWorldsTokenCardApi{
        self.plugin_lol_worlds_token_card_api.as_ref()
    }

    pub fn plugin_manager_api(&self) -> &dyn crate::apis::PluginManagerApi{
        self.plugin_manager_api.as_ref()
    }

    pub fn plugin_manager_diagnostics_api(&self) -> &dyn crate::apis::PluginManagerDiagnosticsApi{
        self.plugin_manager_diagnostics_api.as_ref()
    }

    pub fn plugin_manager_info_api(&self) -> &dyn crate::apis::PluginManagerInfoApi{
        self.plugin_manager_info_api.as_ref()
    }

    pub fn plugin_patcher_api(&self) -> &dyn crate::apis::PluginPatcherApi{
        self.plugin_patcher_api.as_ref()
    }

    pub fn plugin_payments_api(&self) -> &dyn crate::apis::PluginPaymentsApi{
        self.plugin_payments_api.as_ref()
    }

    pub fn plugin_player_notifications_api(&self) -> &dyn crate::apis::PluginPlayerNotificationsApi{
        self.plugin_player_notifications_api.as_ref()
    }

    pub fn plugin_recofriender_api(&self) -> &dyn crate::apis::PluginRecofrienderApi{
        self.plugin_recofriender_api.as_ref()
    }

    pub fn plugin_riot_messaging_service_api(&self) -> &dyn crate::apis::PluginRiotMessagingServiceApi{
        self.plugin_riot_messaging_service_api.as_ref()
    }

    pub fn plugin_sanitizer_api(&self) -> &dyn crate::apis::PluginSanitizerApi{
        self.plugin_sanitizer_api.as_ref()
    }

    pub fn plugin_voice_chat_api(&self) -> &dyn crate::apis::PluginVoiceChatApi{
        self.plugin_voice_chat_api.as_ref()
    }

    pub fn plugins_api(&self) -> &dyn crate::apis::PluginsApi{
        self.plugins_api.as_ref()
    }

    pub fn process_control_api(&self) -> &dyn crate::apis::ProcessControlApi{
        self.process_control_api.as_ref()
    }

    pub fn riotclient_api(&self) -> &dyn crate::apis::RiotclientApi{
        self.riotclient_api.as_ref()
    }

    pub fn system_api(&self) -> &dyn crate::apis::SystemApi{
        self.system_api.as_ref()
    }

    pub fn telemetry_api(&self) -> &dyn crate::apis::TelemetryApi{
        self.telemetry_api.as_ref()
    }

    pub fn tracing_api(&self) -> &dyn crate::apis::TracingApi{
        self.tracing_api.as_ref()
    }

}
