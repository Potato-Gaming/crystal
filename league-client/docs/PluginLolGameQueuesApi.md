# \PluginLolGameQueuesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_game_queues_v1_custom**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_custom) | **Get** /lol-game-queues/v1/custom | 
[**get_lol_game_queues_v1_custom_non_default**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_custom_non_default) | **Get** /lol-game-queues/v1/custom-non-default | 
[**get_lol_game_queues_v1_game_type_config_by_game_type_config_id**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_game_type_config_by_game_type_config_id) | **Get** /lol-game-queues/v1/game-type-config/{gameTypeConfigId} | 
[**get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id) | **Get** /lol-game-queues/v1/game-type-config/{gameTypeConfigId}/map/{mapId} | 
[**get_lol_game_queues_v1_queues**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_queues) | **Get** /lol-game-queues/v1/queues | 
[**get_lol_game_queues_v1_queues_by_id**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_queues_by_id) | **Get** /lol-game-queues/v1/queues/{id} | 
[**get_lol_game_queues_v1_queues_type_by_queue_type**](PluginLolGameQueuesApi.md#get_lol_game_queues_v1_queues_type_by_queue_type) | **Get** /lol-game-queues/v1/queues/type/{queueType} | 



## get_lol_game_queues_v1_custom

> crate::models::LolGameQueuesQueueCustomGame get_lol_game_queues_v1_custom()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolGameQueuesQueueCustomGame**](LolGameQueuesQueueCustomGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_custom_non_default

> crate::models::LolGameQueuesQueueCustomGame get_lol_game_queues_v1_custom_non_default()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolGameQueuesQueueCustomGame**](LolGameQueuesQueueCustomGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_game_type_config_by_game_type_config_id

> crate::models::LolGameQueuesQueueGameTypeConfig get_lol_game_queues_v1_game_type_config_by_game_type_config_id(game_type_config_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_type_config_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolGameQueuesQueueGameTypeConfig**](LolGameQueuesQueueGameTypeConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id

> crate::models::LolGameQueuesQueueGameTypeConfig get_lol_game_queues_v1_game_type_config_by_game_type_config_id_map_by_map_id(game_type_config_id, map_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_type_config_id** | **i32** |  | [required] |
**map_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolGameQueuesQueueGameTypeConfig**](LolGameQueuesQueueGameTypeConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_queues

> Vec<crate::models::LolGameQueuesQueue> get_lol_game_queues_v1_queues()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolGameQueuesQueue>**](LolGameQueuesQueue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_queues_by_id

> crate::models::LolGameQueuesQueue get_lol_game_queues_v1_queues_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::LolGameQueuesQueue**](LolGameQueuesQueue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_queues_v1_queues_type_by_queue_type

> crate::models::LolGameQueuesQueue get_lol_game_queues_v1_queues_type_by_queue_type(queue_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_type** | **String** |  | [required] |

### Return type

[**crate::models::LolGameQueuesQueue**](LolGameQueuesQueue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

