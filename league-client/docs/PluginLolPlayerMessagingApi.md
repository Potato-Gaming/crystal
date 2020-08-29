# \PluginLolPlayerMessagingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_player_messaging_v1_celebration_notification_by_id_acknowledge**](PluginLolPlayerMessagingApi.md#delete_lol_player_messaging_v1_celebration_notification_by_id_acknowledge) | **Delete** /lol-player-messaging/v1/celebration/notification/{id}/acknowledge | 
[**delete_lol_player_messaging_v1_notification_by_id_acknowledge**](PluginLolPlayerMessagingApi.md#delete_lol_player_messaging_v1_notification_by_id_acknowledge) | **Delete** /lol-player-messaging/v1/notification/{id}/acknowledge | 
[**get_lol_player_messaging_v1_celebration_notification**](PluginLolPlayerMessagingApi.md#get_lol_player_messaging_v1_celebration_notification) | **Get** /lol-player-messaging/v1/celebration/notification | 
[**get_lol_player_messaging_v1_notification**](PluginLolPlayerMessagingApi.md#get_lol_player_messaging_v1_notification) | **Get** /lol-player-messaging/v1/notification | 



## delete_lol_player_messaging_v1_celebration_notification_by_id_acknowledge

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_player_messaging_v1_celebration_notification_by_id_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_player_messaging_v1_notification_by_id_acknowledge

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_player_messaging_v1_notification_by_id_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_messaging_v1_celebration_notification

> crate::models::LolPlayerMessagingDynamicCelebrationMessagingNotificationResource get_lol_player_messaging_v1_celebration_notification()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerMessagingDynamicCelebrationMessagingNotificationResource**](LolPlayerMessagingDynamicCelebrationMessagingNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_messaging_v1_notification

> crate::models::LolPlayerMessagingPlayerMessagingNotificationResource get_lol_player_messaging_v1_notification()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerMessagingPlayerMessagingNotificationResource**](LolPlayerMessagingPlayerMessagingNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

