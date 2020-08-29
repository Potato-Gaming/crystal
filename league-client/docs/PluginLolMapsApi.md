# \PluginLolMapsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_maps_v1_map_by_id**](PluginLolMapsApi.md#get_lol_maps_v1_map_by_id) | **Get** /lol-maps/v1/map/{id} | 
[**get_lol_maps_v1_maps**](PluginLolMapsApi.md#get_lol_maps_v1_maps) | **Get** /lol-maps/v1/maps | 
[**get_lol_maps_v2_map_by_id_by_game_mode**](PluginLolMapsApi.md#get_lol_maps_v2_map_by_id_by_game_mode) | **Get** /lol-maps/v2/map/{id}/{gameMode} | 
[**get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator**](PluginLolMapsApi.md#get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator) | **Get** /lol-maps/v2/map/{id}/{gameMode}/{gameMutator} | 
[**get_lol_maps_v2_maps**](PluginLolMapsApi.md#get_lol_maps_v2_maps) | **Get** /lol-maps/v2/maps | 
[**post_lol_maps_v1_map**](PluginLolMapsApi.md#post_lol_maps_v1_map) | **Post** /lol-maps/v1/map | 



## get_lol_maps_v1_map_by_id

> crate::models::LolMapsMaps get_lol_maps_v1_map_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolMapsMaps**](LolMapsMaps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_maps_v1_maps

> Vec<crate::models::LolMapsMaps> get_lol_maps_v1_maps()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolMapsMaps>**](LolMapsMaps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_maps_v2_map_by_id_by_game_mode

> crate::models::LolMapsMaps get_lol_maps_v2_map_by_id_by_game_mode(id, game_mode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**game_mode** | **String** |  | [required] |

### Return type

[**crate::models::LolMapsMaps**](LolMapsMaps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator

> crate::models::LolMapsMaps get_lol_maps_v2_map_by_id_by_game_mode_by_game_mutator(id, game_mode, game_mutator)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**game_mode** | **String** |  | [required] |
**game_mutator** | **String** |  | [required] |

### Return type

[**crate::models::LolMapsMaps**](LolMapsMaps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_maps_v2_maps

> Vec<crate::models::LolMapsMaps> get_lol_maps_v2_maps()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolMapsMaps>**](LolMapsMaps.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_maps_v1_map

> post_lol_maps_v1_map(map)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**map** | [**LolMapsMaps**](LolMapsMaps.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

