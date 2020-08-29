# \PluginPlayerNotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_player_notifications_v1_notifications_by_id**](PluginPlayerNotificationsApi.md#delete_player_notifications_v1_notifications_by_id) | **Delete** /player-notifications/v1/notifications/{id} | 
[**get_player_notifications_v1_config**](PluginPlayerNotificationsApi.md#get_player_notifications_v1_config) | **Get** /player-notifications/v1/config | 
[**get_player_notifications_v1_notifications**](PluginPlayerNotificationsApi.md#get_player_notifications_v1_notifications) | **Get** /player-notifications/v1/notifications | 
[**get_player_notifications_v1_notifications_by_id**](PluginPlayerNotificationsApi.md#get_player_notifications_v1_notifications_by_id) | **Get** /player-notifications/v1/notifications/{id} | 
[**post_player_notifications_v1_notifications**](PluginPlayerNotificationsApi.md#post_player_notifications_v1_notifications) | **Post** /player-notifications/v1/notifications | 
[**put_player_notifications_v1_config**](PluginPlayerNotificationsApi.md#put_player_notifications_v1_config) | **Put** /player-notifications/v1/config | 
[**put_player_notifications_v1_notifications_by_id**](PluginPlayerNotificationsApi.md#put_player_notifications_v1_notifications_by_id) | **Put** /player-notifications/v1/notifications/{id} | 



## delete_player_notifications_v1_notifications_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_player_notifications_v1_notifications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_notifications_v1_config

> crate::models::PlayerNotificationsPlayerNotificationConfigResource get_player_notifications_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PlayerNotificationsPlayerNotificationConfigResource**](PlayerNotificationsPlayerNotificationConfigResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_notifications_v1_notifications

> Vec<crate::models::PlayerNotificationsPlayerNotificationResource> get_player_notifications_v1_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PlayerNotificationsPlayerNotificationResource>**](PlayerNotificationsPlayerNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_player_notifications_v1_notifications_by_id

> crate::models::PlayerNotificationsPlayerNotificationResource get_player_notifications_v1_notifications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::PlayerNotificationsPlayerNotificationResource**](PlayerNotificationsPlayerNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_player_notifications_v1_notifications

> crate::models::PlayerNotificationsPlayerNotificationResource post_player_notifications_v1_notifications(notification)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification** | [**PlayerNotificationsPlayerNotificationResource**](PlayerNotificationsPlayerNotificationResource.md) |  | [required] |

### Return type

[**crate::models::PlayerNotificationsPlayerNotificationResource**](PlayerNotificationsPlayerNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_player_notifications_v1_config

> crate::models::PlayerNotificationsPlayerNotificationConfigResource put_player_notifications_v1_config(config)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**config** | [**PlayerNotificationsPlayerNotificationConfigResource**](PlayerNotificationsPlayerNotificationConfigResource.md) |  | [required] |

### Return type

[**crate::models::PlayerNotificationsPlayerNotificationConfigResource**](PlayerNotificationsPlayerNotificationConfigResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_player_notifications_v1_notifications_by_id

> crate::models::PlayerNotificationsPlayerNotificationResource put_player_notifications_v1_notifications_by_id(id, notification)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**notification** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::PlayerNotificationsPlayerNotificationResource**](PlayerNotificationsPlayerNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

