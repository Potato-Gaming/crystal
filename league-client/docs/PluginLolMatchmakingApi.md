# \PluginLolMatchmakingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_matchmaking_v1_search**](PluginLolMatchmakingApi.md#delete_lol_matchmaking_v1_search) | **Delete** /lol-matchmaking/v1/search | 
[**get_lol_matchmaking_v1_ready_check**](PluginLolMatchmakingApi.md#get_lol_matchmaking_v1_ready_check) | **Get** /lol-matchmaking/v1/ready-check | 
[**get_lol_matchmaking_v1_search**](PluginLolMatchmakingApi.md#get_lol_matchmaking_v1_search) | **Get** /lol-matchmaking/v1/search | 
[**get_lol_matchmaking_v1_search_errors**](PluginLolMatchmakingApi.md#get_lol_matchmaking_v1_search_errors) | **Get** /lol-matchmaking/v1/search/errors | 
[**get_lol_matchmaking_v1_search_errors_by_id**](PluginLolMatchmakingApi.md#get_lol_matchmaking_v1_search_errors_by_id) | **Get** /lol-matchmaking/v1/search/errors/{id} | 
[**post_lol_matchmaking_v1_ready_check_accept**](PluginLolMatchmakingApi.md#post_lol_matchmaking_v1_ready_check_accept) | **Post** /lol-matchmaking/v1/ready-check/accept | 
[**post_lol_matchmaking_v1_ready_check_decline**](PluginLolMatchmakingApi.md#post_lol_matchmaking_v1_ready_check_decline) | **Post** /lol-matchmaking/v1/ready-check/decline | 
[**post_lol_matchmaking_v1_search**](PluginLolMatchmakingApi.md#post_lol_matchmaking_v1_search) | **Post** /lol-matchmaking/v1/search | 
[**put_lol_matchmaking_v1_search**](PluginLolMatchmakingApi.md#put_lol_matchmaking_v1_search) | **Put** /lol-matchmaking/v1/search | 



## delete_lol_matchmaking_v1_search

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_matchmaking_v1_search()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_matchmaking_v1_ready_check

> crate::models::LolMatchmakingMatchmakingReadyCheckResource get_lol_matchmaking_v1_ready_check()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolMatchmakingMatchmakingReadyCheckResource**](LolMatchmakingMatchmakingReadyCheckResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_matchmaking_v1_search

> crate::models::LolMatchmakingMatchmakingSearchResource get_lol_matchmaking_v1_search()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolMatchmakingMatchmakingSearchResource**](LolMatchmakingMatchmakingSearchResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_matchmaking_v1_search_errors

> Vec<crate::models::LolMatchmakingMatchmakingSearchErrorResource> get_lol_matchmaking_v1_search_errors()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolMatchmakingMatchmakingSearchErrorResource>**](LolMatchmakingMatchmakingSearchErrorResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_matchmaking_v1_search_errors_by_id

> crate::models::LolMatchmakingMatchmakingSearchErrorResource get_lol_matchmaking_v1_search_errors_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::LolMatchmakingMatchmakingSearchErrorResource**](LolMatchmakingMatchmakingSearchErrorResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_matchmaking_v1_ready_check_accept

> ::std::collections::HashMap<String, serde_json::Value> post_lol_matchmaking_v1_ready_check_accept()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_matchmaking_v1_ready_check_decline

> ::std::collections::HashMap<String, serde_json::Value> post_lol_matchmaking_v1_ready_check_decline()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_matchmaking_v1_search

> ::std::collections::HashMap<String, serde_json::Value> post_lol_matchmaking_v1_search()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_matchmaking_v1_search

> ::std::collections::HashMap<String, serde_json::Value> put_lol_matchmaking_v1_search(search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | [**LolMatchmakingMatchmakingSearchResource**](LolMatchmakingMatchmakingSearchResource.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

