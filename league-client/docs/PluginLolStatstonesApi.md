# \PluginLolStatstonesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_statstones_v1_eog_notifications**](PluginLolStatstonesApi.md#delete_lol_statstones_v1_eog_notifications) | **Delete** /lol-statstones/v1/eog-notifications | 
[**delete_lol_statstones_v1_eog_notifications_by_key**](PluginLolStatstonesApi.md#delete_lol_statstones_v1_eog_notifications_by_key) | **Delete** /lol-statstones/v1/eog-notifications/{key} | 
[**delete_lol_statstones_v1_vignette_notifications**](PluginLolStatstonesApi.md#delete_lol_statstones_v1_vignette_notifications) | **Delete** /lol-statstones/v1/vignette-notifications | 
[**delete_lol_statstones_v1_vignette_notifications_by_key**](PluginLolStatstonesApi.md#delete_lol_statstones_v1_vignette_notifications_by_key) | **Delete** /lol-statstones/v1/vignette-notifications/{key} | 
[**get_lol_statstones_v1_eog_notifications**](PluginLolStatstonesApi.md#get_lol_statstones_v1_eog_notifications) | **Get** /lol-statstones/v1/eog-notifications | 
[**get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id**](PluginLolStatstonesApi.md#get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id) | **Get** /lol-statstones/v1/featured-champion-statstones/{championItemId} | 
[**get_lol_statstones_v1_profile_summary_by_puuid**](PluginLolStatstonesApi.md#get_lol_statstones_v1_profile_summary_by_puuid) | **Get** /lol-statstones/v1/profile-summary/{puuid} | 
[**get_lol_statstones_v1_statstone_by_content_id_owned**](PluginLolStatstonesApi.md#get_lol_statstones_v1_statstone_by_content_id_owned) | **Get** /lol-statstones/v1/statstone/{contentId}/owned | 
[**get_lol_statstones_v1_statstones_enabled_queue_ids**](PluginLolStatstonesApi.md#get_lol_statstones_v1_statstones_enabled_queue_ids) | **Get** /lol-statstones/v1/statstones-enabled-queue-ids | 
[**get_lol_statstones_v1_vignette_notifications**](PluginLolStatstonesApi.md#get_lol_statstones_v1_vignette_notifications) | **Get** /lol-statstones/v1/vignette-notifications | 
[**get_lol_statstones_v2_player_statstones_self_by_champion_item_id**](PluginLolStatstonesApi.md#get_lol_statstones_v2_player_statstones_self_by_champion_item_id) | **Get** /lol-statstones/v2/player-statstones-self/{championItemId} | 
[**get_lol_statstones_v2_player_summary_self**](PluginLolStatstonesApi.md#get_lol_statstones_v2_player_summary_self) | **Get** /lol-statstones/v2/player-summary-self | 
[**post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id**](PluginLolStatstonesApi.md#post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id) | **Post** /lol-statstones/v1/featured-champion-statstones/{championItemId}/{statstoneId} | 



## delete_lol_statstones_v1_eog_notifications

> delete_lol_statstones_v1_eog_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_statstones_v1_eog_notifications_by_key

> delete_lol_statstones_v1_eog_notifications_by_key(key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_statstones_v1_vignette_notifications

> delete_lol_statstones_v1_vignette_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_statstones_v1_vignette_notifications_by_key

> delete_lol_statstones_v1_vignette_notifications_by_key(key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_eog_notifications

> Vec<serde_json::Value> get_lol_statstones_v1_eog_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id

> Vec<crate::models::LolStatstonesStatstone> get_lol_statstones_v1_featured_champion_statstones_by_champion_item_id(champion_item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**champion_item_id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::LolStatstonesStatstone>**](LolStatstonesStatstone.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_profile_summary_by_puuid

> Vec<crate::models::LolStatstonesProfileStatstoneSummary> get_lol_statstones_v1_profile_summary_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolStatstonesProfileStatstoneSummary>**](LolStatstonesProfileStatstoneSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_statstone_by_content_id_owned

> bool get_lol_statstones_v1_statstone_by_content_id_owned(content_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_id** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_statstones_enabled_queue_ids

> Vec<i32> get_lol_statstones_v1_statstones_enabled_queue_ids()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v1_vignette_notifications

> Vec<serde_json::Value> get_lol_statstones_v1_vignette_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v2_player_statstones_self_by_champion_item_id

> Vec<crate::models::LolStatstonesStatstoneSet> get_lol_statstones_v2_player_statstones_self_by_champion_item_id(champion_item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**champion_item_id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::LolStatstonesStatstoneSet>**](LolStatstonesStatstoneSet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_statstones_v2_player_summary_self

> Vec<crate::models::LolStatstonesChampionStatstoneSummary> get_lol_statstones_v2_player_summary_self()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolStatstonesChampionStatstoneSummary>**](LolStatstonesChampionStatstoneSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id

> ::std::collections::HashMap<String, serde_json::Value> post_lol_statstones_v1_featured_champion_statstones_by_champion_item_id_by_statstone_id(champion_item_id, statstone_id, featured_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**champion_item_id** | **i32** |  | [required] |
**statstone_id** | **String** |  | [required] |
**featured_info** | [**LolStatstonesStatstoneFeaturedRequest**](LolStatstonesStatstoneFeaturedRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

