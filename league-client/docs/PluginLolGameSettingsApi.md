# \PluginLolGameSettingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_game_settings_v1_didreset**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_didreset) | **Get** /lol-game-settings/v1/didreset | 
[**get_lol_game_settings_v1_game_settings**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_game_settings) | **Get** /lol-game-settings/v1/game-settings | 
[**get_lol_game_settings_v1_game_settings_schema**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_game_settings_schema) | **Get** /lol-game-settings/v1/game-settings-schema | 
[**get_lol_game_settings_v1_input_settings**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_input_settings) | **Get** /lol-game-settings/v1/input-settings | 
[**get_lol_game_settings_v1_input_settings_schema**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_input_settings_schema) | **Get** /lol-game-settings/v1/input-settings-schema | 
[**get_lol_game_settings_v1_ready**](PluginLolGameSettingsApi.md#get_lol_game_settings_v1_ready) | **Get** /lol-game-settings/v1/ready | 
[**patch_lol_game_settings_v1_game_settings**](PluginLolGameSettingsApi.md#patch_lol_game_settings_v1_game_settings) | **Patch** /lol-game-settings/v1/game-settings | 
[**patch_lol_game_settings_v1_input_settings**](PluginLolGameSettingsApi.md#patch_lol_game_settings_v1_input_settings) | **Patch** /lol-game-settings/v1/input-settings | 
[**post_lol_game_settings_v1_reload_post_game**](PluginLolGameSettingsApi.md#post_lol_game_settings_v1_reload_post_game) | **Post** /lol-game-settings/v1/reload-post-game | 
[**post_lol_game_settings_v1_save**](PluginLolGameSettingsApi.md#post_lol_game_settings_v1_save) | **Post** /lol-game-settings/v1/save | 



## get_lol_game_settings_v1_didreset

> bool get_lol_game_settings_v1_didreset()


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


## get_lol_game_settings_v1_game_settings

> ::std::collections::HashMap<String, serde_json::Value> get_lol_game_settings_v1_game_settings()


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


## get_lol_game_settings_v1_game_settings_schema

> ::std::collections::HashMap<String, serde_json::Value> get_lol_game_settings_v1_game_settings_schema()


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


## get_lol_game_settings_v1_input_settings

> ::std::collections::HashMap<String, serde_json::Value> get_lol_game_settings_v1_input_settings()


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


## get_lol_game_settings_v1_input_settings_schema

> ::std::collections::HashMap<String, serde_json::Value> get_lol_game_settings_v1_input_settings_schema()


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


## get_lol_game_settings_v1_ready

> bool get_lol_game_settings_v1_ready()


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


## patch_lol_game_settings_v1_game_settings

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_game_settings_v1_game_settings(settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_resource** | **serde_json::Value** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_game_settings_v1_input_settings

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_game_settings_v1_input_settings(settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_resource** | **serde_json::Value** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_game_settings_v1_reload_post_game

> post_lol_game_settings_v1_reload_post_game()


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


## post_lol_game_settings_v1_save

> bool post_lol_game_settings_v1_save()


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

