# \PluginLolPlayerLevelUpApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_player_level_up_v1_level_up**](PluginLolPlayerLevelUpApi.md#get_lol_player_level_up_v1_level_up) | **Get** /lol-player-level-up/v1/level-up | 
[**get_lol_player_level_up_v1_level_up_notifications_by_plugin_name**](PluginLolPlayerLevelUpApi.md#get_lol_player_level_up_v1_level_up_notifications_by_plugin_name) | **Get** /lol-player-level-up/v1/level-up-notifications/{pluginName} | 
[**post_lol_player_level_up_v1_level_up_notifications_by_plugin_name**](PluginLolPlayerLevelUpApi.md#post_lol_player_level_up_v1_level_up_notifications_by_plugin_name) | **Post** /lol-player-level-up/v1/level-up-notifications/{pluginName} | 



## get_lol_player_level_up_v1_level_up

> crate::models::LolPlayerLevelUpPlayerLevelUpEvent get_lol_player_level_up_v1_level_up()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerLevelUpPlayerLevelUpEvent**](LolPlayerLevelUpPlayerLevelUpEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_level_up_v1_level_up_notifications_by_plugin_name

> crate::models::LolPlayerLevelUpPlayerLevelUpEventAck get_lol_player_level_up_v1_level_up_notifications_by_plugin_name(plugin_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |

### Return type

[**crate::models::LolPlayerLevelUpPlayerLevelUpEventAck**](LolPlayerLevelUpPlayerLevelUpEventAck.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_player_level_up_v1_level_up_notifications_by_plugin_name

> post_lol_player_level_up_v1_level_up_notifications_by_plugin_name(plugin_name, level_up_event_ack)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin_name** | **String** |  | [required] |
**level_up_event_ack** | [**LolPlayerLevelUpPlayerLevelUpEventAck**](LolPlayerLevelUpPlayerLevelUpEventAck.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

