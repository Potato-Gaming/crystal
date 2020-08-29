# \PluginLolCareerStatsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue) | **Get** /lol-career-stats/v1/champion-averages/{championId}/{position}/{tier}/{queue} | 
[**get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue) | **Get** /lol-career-stats/v1/champion-averages/season/{season}/{championId}/{position}/{tier}/{queue} | 
[**get_lol_career_stats_v1_champion_experts_by_champion_id_by_position**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_champion_experts_by_champion_id_by_position) | **Get** /lol-career-stats/v1/champion-experts/{championId}/{position} | 
[**get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position) | **Get** /lol-career-stats/v1/champion-experts/season/{season}/{championId}/{position} | 
[**get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue) | **Get** /lol-career-stats/v1/position-averages/{position}/{tier}/{queue} | 
[**get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue) | **Get** /lol-career-stats/v1/position-averages/season/{season}/{position}/{tier}/{queue} | 
[**get_lol_career_stats_v1_position_experts_by_position**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_position_experts_by_position) | **Get** /lol-career-stats/v1/position-experts/{position} | 
[**get_lol_career_stats_v1_position_experts_season_by_season_by_position**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_position_experts_season_by_season_by_position) | **Get** /lol-career-stats/v1/position-experts/season/{season}/{position} | 
[**get_lol_career_stats_v1_summoner_games_by_puuid**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_summoner_games_by_puuid) | **Get** /lol-career-stats/v1/summoner-games/{puuid} | 
[**get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season) | **Get** /lol-career-stats/v1/summoner-games/{puuid}/season/{season} | 
[**get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position**](PluginLolCareerStatsApi.md#get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position) | **Get** /lol-career-stats/v1/summoner-stats/{puuid}/{season}/{queue}/{position} | 
[**post_lol_career_stats_v1_champion_stats_percentiles**](PluginLolCareerStatsApi.md#post_lol_career_stats_v1_champion_stats_percentiles) | **Post** /lol-career-stats/v1/champion-stats-percentiles | 
[**post_lol_career_stats_v1_position_stats_percentiles**](PluginLolCareerStatsApi.md#post_lol_career_stats_v1_position_stats_percentiles) | **Post** /lol-career-stats/v1/position-stats-percentiles | 



## get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue

> crate::models::LolCareerStatsChampionQueueStatsResponse get_lol_career_stats_v1_champion_averages_by_champion_id_by_position_by_tier_by_queue(champion_id, position, tier, queue)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**champion_id** | **i32** |  | [required] |
**position** | **String** |  | [required] |
**tier** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

[**crate::models::LolCareerStatsChampionQueueStatsResponse**](LolCareerStatsChampionQueueStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue

> crate::models::LolCareerStatsChampionQueueStatsResponse get_lol_career_stats_v1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue(season, champion_id, position, tier, queue)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season** | **i32** |  | [required] |
**champion_id** | **i32** |  | [required] |
**position** | **String** |  | [required] |
**tier** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

[**crate::models::LolCareerStatsChampionQueueStatsResponse**](LolCareerStatsChampionQueueStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_champion_experts_by_champion_id_by_position

> Vec<crate::models::LolCareerStatsExpertPlayer> get_lol_career_stats_v1_champion_experts_by_champion_id_by_position(champion_id, position)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**champion_id** | **i32** |  | [required] |
**position** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsExpertPlayer>**](LolCareerStatsExpertPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position

> Vec<crate::models::LolCareerStatsExpertPlayer> get_lol_career_stats_v1_champion_experts_season_by_season_by_champion_id_by_position(season, champion_id, position)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season** | **i32** |  | [required] |
**champion_id** | **i32** |  | [required] |
**position** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsExpertPlayer>**](LolCareerStatsExpertPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue

> crate::models::LolCareerStatsChampionQueueStatsResponse get_lol_career_stats_v1_position_averages_by_position_by_tier_by_queue(position, tier, queue)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position** | **String** |  | [required] |
**tier** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

[**crate::models::LolCareerStatsChampionQueueStatsResponse**](LolCareerStatsChampionQueueStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue

> crate::models::LolCareerStatsChampionQueueStatsResponse get_lol_career_stats_v1_position_averages_season_by_season_by_position_by_tier_by_queue(season, position, tier, queue)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season** | **i32** |  | [required] |
**position** | **String** |  | [required] |
**tier** | **String** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

[**crate::models::LolCareerStatsChampionQueueStatsResponse**](LolCareerStatsChampionQueueStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_position_experts_by_position

> Vec<crate::models::LolCareerStatsExpertPlayer> get_lol_career_stats_v1_position_experts_by_position(position)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**position** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsExpertPlayer>**](LolCareerStatsExpertPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_position_experts_season_by_season_by_position

> Vec<crate::models::LolCareerStatsExpertPlayer> get_lol_career_stats_v1_position_experts_season_by_season_by_position(season, position)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**season** | **i32** |  | [required] |
**position** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsExpertPlayer>**](LolCareerStatsExpertPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_summoner_games_by_puuid

> ::std::collections::HashMap<String, serde_json::Value> get_lol_career_stats_v1_summoner_games_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season

> ::std::collections::HashMap<String, serde_json::Value> get_lol_career_stats_v1_summoner_games_by_puuid_season_by_season(puuid, season)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**season** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position

> ::std::collections::HashMap<String, serde_json::Value> get_lol_career_stats_v1_summoner_stats_by_puuid_by_season_by_queue_by_position(puuid, season, queue, position, champion_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**season** | **i32** |  | [required] |
**queue** | **String** |  | [required] |
**position** | **String** |  | [required] |
**champion_id** | Option<**i32**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_career_stats_v1_champion_stats_percentiles

> Vec<crate::models::LolCareerStatsStatisticsPercentilesResponse> post_lol_career_stats_v1_champion_stats_percentiles(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::LolCareerStatsStatsQueryRequest>**](LolCareerStatsStatsQueryRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsStatisticsPercentilesResponse>**](LolCareerStatsStatisticsPercentilesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_career_stats_v1_position_stats_percentiles

> Vec<crate::models::LolCareerStatsStatisticsPercentilesResponse> post_lol_career_stats_v1_position_stats_percentiles(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**Vec<crate::models::LolCareerStatsPositionStatsQueryRequest>**](LolCareerStatsPositionStatsQueryRequest.md) |  | [required] |

### Return type

[**Vec<crate::models::LolCareerStatsStatisticsPercentilesResponse>**](LolCareerStatsStatisticsPercentilesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

