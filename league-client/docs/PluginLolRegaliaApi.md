# \PluginLolRegaliaApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_regalia_v2_config**](PluginLolRegaliaApi.md#get_lol_regalia_v2_config) | **Get** /lol-regalia/v2/config | 
[**get_lol_regalia_v2_current_summoner_regalia**](PluginLolRegaliaApi.md#get_lol_regalia_v2_current_summoner_regalia) | **Get** /lol-regalia/v2/current-summoner/regalia | 
[**get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia**](PluginLolRegaliaApi.md#get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia) | **Get** /lol-regalia/v2/summoners/{summonerId}/queues/{queue}/positions/{position}/regalia | 
[**get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia**](PluginLolRegaliaApi.md#get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia) | **Get** /lol-regalia/v2/summoners/{summonerId}/queues/{queue}/regalia | 
[**get_lol_regalia_v2_summoners_by_summoner_id_regalia**](PluginLolRegaliaApi.md#get_lol_regalia_v2_summoners_by_summoner_id_regalia) | **Get** /lol-regalia/v2/summoners/{summonerId}/regalia | 
[**get_lol_regalia_v2_summoners_by_summoner_id_regalia_async**](PluginLolRegaliaApi.md#get_lol_regalia_v2_summoners_by_summoner_id_regalia_async) | **Get** /lol-regalia/v2/summoners/{summonerId}/regalia/async | 
[**put_lol_regalia_v2_current_summoner_regalia**](PluginLolRegaliaApi.md#put_lol_regalia_v2_current_summoner_regalia) | **Put** /lol-regalia/v2/current-summoner/regalia | 



## get_lol_regalia_v2_config

> crate::models::LolRegaliaRegaliaFrontendConfig get_lol_regalia_v2_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRegaliaRegaliaFrontendConfig**](LolRegaliaRegaliaFrontendConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_regalia_v2_current_summoner_regalia

> crate::models::LolRegaliaRegaliaWithPreferences get_lol_regalia_v2_current_summoner_regalia()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolRegaliaRegaliaWithPreferences**](LolRegaliaRegaliaWithPreferences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia

> crate::models::LolRegaliaRegalia get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(summoner_id, queue, position)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**queue** | **String** |  | [required] |
**position** | **String** |  | [required] |

### Return type

[**crate::models::LolRegaliaRegalia**](LolRegaliaRegalia.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia

> crate::models::LolRegaliaRegalia get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia(summoner_id, queue)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**queue** | **String** |  | [required] |

### Return type

[**crate::models::LolRegaliaRegalia**](LolRegaliaRegalia.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_regalia_v2_summoners_by_summoner_id_regalia

> crate::models::LolRegaliaRegalia get_lol_regalia_v2_summoners_by_summoner_id_regalia(summoner_id, hovercard)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**hovercard** | **bool** |  | [required] |

### Return type

[**crate::models::LolRegaliaRegalia**](LolRegaliaRegalia.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_regalia_v2_summoners_by_summoner_id_regalia_async

> crate::models::LolRegaliaRegaliaAsync get_lol_regalia_v2_summoners_by_summoner_id_regalia_async(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolRegaliaRegaliaAsync**](LolRegaliaRegaliaAsync.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_regalia_v2_current_summoner_regalia

> crate::models::LolRegaliaRegaliaWithPreferences put_lol_regalia_v2_current_summoner_regalia(regalia)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**regalia** | [**LolRegaliaRegaliaPreferences**](LolRegaliaRegaliaPreferences.md) |  | [required] |

### Return type

[**crate::models::LolRegaliaRegaliaWithPreferences**](LolRegaliaRegaliaWithPreferences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

