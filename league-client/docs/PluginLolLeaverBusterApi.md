# \PluginLolLeaverBusterApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_leaver_buster_v1_notifications_by_id**](PluginLolLeaverBusterApi.md#delete_lol_leaver_buster_v1_notifications_by_id) | **Delete** /lol-leaver-buster/v1/notifications/{id} | 
[**get_lol_leaver_buster_v1_notifications**](PluginLolLeaverBusterApi.md#get_lol_leaver_buster_v1_notifications) | **Get** /lol-leaver-buster/v1/notifications | 
[**get_lol_leaver_buster_v1_notifications_by_id**](PluginLolLeaverBusterApi.md#get_lol_leaver_buster_v1_notifications_by_id) | **Get** /lol-leaver-buster/v1/notifications/{id} | 



## delete_lol_leaver_buster_v1_notifications_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_leaver_buster_v1_notifications_by_id(id)


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


## get_lol_leaver_buster_v1_notifications

> Vec<crate::models::LolLeaverBusterLeaverBusterNotificationResource> get_lol_leaver_buster_v1_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLeaverBusterLeaverBusterNotificationResource>**](LolLeaverBusterLeaverBusterNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_leaver_buster_v1_notifications_by_id

> crate::models::LolLeaverBusterLeaverBusterNotificationResource get_lol_leaver_buster_v1_notifications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::LolLeaverBusterLeaverBusterNotificationResource**](LolLeaverBusterLeaverBusterNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

