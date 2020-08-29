# \PluginLolPlayerPreferencesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_player_preferences_v1_player_preferences_ready**](PluginLolPlayerPreferencesApi.md#get_lol_player_preferences_v1_player_preferences_ready) | **Get** /lol-player-preferences/v1/player-preferences-ready | 
[**get_lol_player_preferences_v1_preference_by_type**](PluginLolPlayerPreferencesApi.md#get_lol_player_preferences_v1_preference_by_type) | **Get** /lol-player-preferences/v1/preference/{type} | 
[**post_lol_player_preferences_v1_hash**](PluginLolPlayerPreferencesApi.md#post_lol_player_preferences_v1_hash) | **Post** /lol-player-preferences/v1/hash | 
[**post_lol_player_preferences_v1_player_preferences_endpoint_override**](PluginLolPlayerPreferencesApi.md#post_lol_player_preferences_v1_player_preferences_endpoint_override) | **Post** /lol-player-preferences/v1/player-preferences-endpoint-override | 
[**put_lol_player_preferences_v1_preference**](PluginLolPlayerPreferencesApi.md#put_lol_player_preferences_v1_preference) | **Put** /lol-player-preferences/v1/preference | 



## get_lol_player_preferences_v1_player_preferences_ready

> bool get_lol_player_preferences_v1_player_preferences_ready()


### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_preferences_v1_preference_by_type

> ::std::collections::HashMap<String, serde_json::Value> get_lol_player_preferences_v1_preference_by_type(_type, hash)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**hash** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_player_preferences_v1_hash

> String post_lol_player_preferences_v1_hash(preferences)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preferences** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_player_preferences_v1_player_preferences_endpoint_override

> ::std::collections::HashMap<String, serde_json::Value> post_lol_player_preferences_v1_player_preferences_endpoint_override(preferences)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preferences** | [**LolPlayerPreferencesPlayerPreferencesEndpoint**](LolPlayerPreferencesPlayerPreferencesEndpoint.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_player_preferences_v1_preference

> ::std::collections::HashMap<String, serde_json::Value> put_lol_player_preferences_v1_preference(preferences)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preferences** | [**LolPlayerPreferencesPlayerPreferences**](LolPlayerPreferencesPlayerPreferences.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

