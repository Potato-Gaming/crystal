# \PluginLolRankedApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier**](PluginLolRankedApi.md#get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier) | **Get** /lol-ranked/v1/apex-leagues/{queueType}/{tier} | 
[**get_lol_ranked_v1_current_lp_change_notification**](PluginLolRankedApi.md#get_lol_ranked_v1_current_lp_change_notification) | **Get** /lol-ranked/v1/current-lp-change-notification | 
[**get_lol_ranked_v1_current_ranked_stats**](PluginLolRankedApi.md#get_lol_ranked_v1_current_ranked_stats) | **Get** /lol-ranked/v1/current-ranked-stats | 
[**get_lol_ranked_v1_eos_notifications**](PluginLolRankedApi.md#get_lol_ranked_v1_eos_notifications) | **Get** /lol-ranked/v1/eos-notifications | 
[**get_lol_ranked_v1_eos_rewards**](PluginLolRankedApi.md#get_lol_ranked_v1_eos_rewards) | **Get** /lol-ranked/v1/eos-rewards | 
[**get_lol_ranked_v1_league_ladders_by_puuid**](PluginLolRankedApi.md#get_lol_ranked_v1_league_ladders_by_puuid) | **Get** /lol-ranked/v1/league-ladders/{puuid} | 
[**get_lol_ranked_v1_notifications**](PluginLolRankedApi.md#get_lol_ranked_v1_notifications) | **Get** /lol-ranked/v1/notifications | 
[**get_lol_ranked_v1_ranked_stats**](PluginLolRankedApi.md#get_lol_ranked_v1_ranked_stats) | **Get** /lol-ranked/v1/ranked-stats | 
[**get_lol_ranked_v1_ranked_stats_by_puuid**](PluginLolRankedApi.md#get_lol_ranked_v1_ranked_stats_by_puuid) | **Get** /lol-ranked/v1/ranked-stats/{puuid} | 
[**get_lol_ranked_v1_signed_ranked_stats**](PluginLolRankedApi.md#get_lol_ranked_v1_signed_ranked_stats) | **Get** /lol-ranked/v1/signed-ranked-stats | 
[**get_lol_ranked_v1_splits_config**](PluginLolRankedApi.md#get_lol_ranked_v1_splits_config) | **Get** /lol-ranked/v1/splits-config | 
[**get_lol_ranked_v2_tiers**](PluginLolRankedApi.md#get_lol_ranked_v2_tiers) | **Get** /lol-ranked/v2/tiers | 
[**post_lol_ranked_v1_eos_notifications_by_id_acknowledge**](PluginLolRankedApi.md#post_lol_ranked_v1_eos_notifications_by_id_acknowledge) | **Post** /lol-ranked/v1/eos-notifications/{id}/acknowledge | 
[**post_lol_ranked_v1_notifications_by_id_acknowledge**](PluginLolRankedApi.md#post_lol_ranked_v1_notifications_by_id_acknowledge) | **Post** /lol-ranked/v1/notifications/{id}/acknowledge | 



## get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier

> crate::models::LolRankedLeagueLadderInfo get_lol_ranked_v1_apex_leagues_by_queue_type_by_tier(queue_type, tier)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_type** | **String** |  | [required] |
**tier** | **String** |  | [required] |

### Return type

[**crate::models::LolRankedLeagueLadderInfo**](LolRankedLeagueLadderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_current_lp_change_notification

> crate::models::LolRankedLcuLeagueNotification get_lol_ranked_v1_current_lp_change_notification()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRankedLcuLeagueNotification**](LolRankedLcuLeagueNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_current_ranked_stats

> crate::models::LolRankedRankedStats get_lol_ranked_v1_current_ranked_stats()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRankedRankedStats**](LolRankedRankedStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_eos_notifications

> Vec<crate::models::LolRankedEosNotificationResource> get_lol_ranked_v1_eos_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolRankedEosNotificationResource>**](LolRankedEosNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_eos_rewards

> crate::models::LolRankedEosRewardsConfig get_lol_ranked_v1_eos_rewards()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRankedEosRewardsConfig**](LolRankedEosRewardsConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_league_ladders_by_puuid

> Vec<crate::models::LolRankedLeagueLadderInfo> get_lol_ranked_v1_league_ladders_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolRankedLeagueLadderInfo>**](LolRankedLeagueLadderInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_notifications

> Vec<crate::models::LolRankedLcuLeagueNotification> get_lol_ranked_v1_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolRankedLcuLeagueNotification>**](LolRankedLcuLeagueNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_ranked_stats

> ::std::collections::HashMap<String, crate::models::LolRankedRankedStats> get_lol_ranked_v1_ranked_stats(puuids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::LolRankedRankedStats>**](LolRankedRankedStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_ranked_stats_by_puuid

> crate::models::LolRankedRankedStats get_lol_ranked_v1_ranked_stats_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolRankedRankedStats**](LolRankedRankedStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_signed_ranked_stats

> crate::models::SignedRankedStatsDto get_lol_ranked_v1_signed_ranked_stats()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SignedRankedStatsDto**](SignedRankedStatsDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v1_splits_config

> crate::models::LolRankedRewardsInfo get_lol_ranked_v1_splits_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRankedRewardsInfo**](LolRankedRewardsInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_ranked_v2_tiers

> Vec<crate::models::LolRankedParticipantTiers> get_lol_ranked_v2_tiers(summoner_ids, queue_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_ids** | [**Vec<i64>**](i64.md) |  | [required] |
**queue_types** | [**Vec<crate::models::LolRankedLeagueQueueType>**](crate::models::LolRankedLeagueQueueType.md) |  | [required] |

### Return type

[**Vec<crate::models::LolRankedParticipantTiers>**](LolRankedParticipantTiers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_ranked_v1_eos_notifications_by_id_acknowledge

> ::std::collections::HashMap<String, serde_json::Value> post_lol_ranked_v1_eos_notifications_by_id_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_ranked_v1_notifications_by_id_acknowledge

> ::std::collections::HashMap<String, serde_json::Value> post_lol_ranked_v1_notifications_by_id_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

