# \PluginLolReplaysApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_replays_v1_configuration**](PluginLolReplaysApi.md#get_lol_replays_v1_configuration) | **Get** /lol-replays/v1/configuration | 
[**get_lol_replays_v1_metadata_by_game_id**](PluginLolReplaysApi.md#get_lol_replays_v1_metadata_by_game_id) | **Get** /lol-replays/v1/metadata/{gameId} | 
[**get_lol_replays_v1_rofls_path**](PluginLolReplaysApi.md#get_lol_replays_v1_rofls_path) | **Get** /lol-replays/v1/rofls/path | 
[**get_lol_replays_v1_rofls_path_default**](PluginLolReplaysApi.md#get_lol_replays_v1_rofls_path_default) | **Get** /lol-replays/v1/rofls/path/default | 
[**post_lol_replays_v1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id**](PluginLolReplaysApi.md#post_lol_replays_v1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id) | **Post** /lol-replays/v1/metadata/{gameId}/create/gameVersion/{gameVersion}/gameType/{gameType}/queueId/{queueId} | 
[**post_lol_replays_v1_rofls_by_game_id_download**](PluginLolReplaysApi.md#post_lol_replays_v1_rofls_by_game_id_download) | **Post** /lol-replays/v1/rofls/{gameId}/download | 
[**post_lol_replays_v1_rofls_by_game_id_download_graceful**](PluginLolReplaysApi.md#post_lol_replays_v1_rofls_by_game_id_download_graceful) | **Post** /lol-replays/v1/rofls/{gameId}/download/graceful | 
[**post_lol_replays_v1_rofls_by_game_id_watch**](PluginLolReplaysApi.md#post_lol_replays_v1_rofls_by_game_id_watch) | **Post** /lol-replays/v1/rofls/{gameId}/watch | 
[**post_lol_replays_v1_rofls_scan**](PluginLolReplaysApi.md#post_lol_replays_v1_rofls_scan) | **Post** /lol-replays/v1/rofls/scan | 
[**post_lol_replays_v2_metadata_by_game_id_create**](PluginLolReplaysApi.md#post_lol_replays_v2_metadata_by_game_id_create) | **Post** /lol-replays/v2/metadata/{gameId}/create | 



## get_lol_replays_v1_configuration

> crate::models::LolReplaysReplaysConfiguration get_lol_replays_v1_configuration()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolReplaysReplaysConfiguration**](LolReplaysReplaysConfiguration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_replays_v1_metadata_by_game_id

> crate::models::LolReplaysReplayMetadata get_lol_replays_v1_metadata_by_game_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolReplaysReplayMetadata**](LolReplaysReplayMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_replays_v1_rofls_path

> String get_lol_replays_v1_rofls_path()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_replays_v1_rofls_path_default

> String get_lol_replays_v1_rofls_path_default()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_replays_v1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id

> post_lol_replays_v1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id(game_id, game_version, game_type, queue_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |
**game_version** | **String** |  | [required] |
**game_type** | **String** |  | [required] |
**queue_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_replays_v1_rofls_by_game_id_download

> post_lol_replays_v1_rofls_by_game_id_download(game_id, context_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |
**context_data** | [**LolReplaysReplayContextData**](LolReplaysReplayContextData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_replays_v1_rofls_by_game_id_download_graceful

> post_lol_replays_v1_rofls_by_game_id_download_graceful(game_id, context_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |
**context_data** | [**LolReplaysReplayContextData**](LolReplaysReplayContextData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_replays_v1_rofls_by_game_id_watch

> post_lol_replays_v1_rofls_by_game_id_watch(game_id, context_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |
**context_data** | [**LolReplaysReplayContextData**](LolReplaysReplayContextData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_replays_v1_rofls_scan

> post_lol_replays_v1_rofls_scan()


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


## post_lol_replays_v2_metadata_by_game_id_create

> post_lol_replays_v2_metadata_by_game_id_create(game_id, request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |
**request** | [**LolReplaysReplayCreateMetadata**](LolReplaysReplayCreateMetadata.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

