# \PluginLolKrShutdownLawApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_kr_shutdown_law_v1_custom_status**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_custom_status) | **Get** /lol-kr-shutdown-law/v1/custom-status | 
[**get_lol_kr_shutdown_law_v1_disabled_queues**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_disabled_queues) | **Get** /lol-kr-shutdown-law/v1/disabled-queues | 
[**get_lol_kr_shutdown_law_v1_notification**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_notification) | **Get** /lol-kr-shutdown-law/v1/notification | 
[**get_lol_kr_shutdown_law_v1_queue_status_by_queue_id**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_queue_status_by_queue_id) | **Get** /lol-kr-shutdown-law/v1/queue-status/{queue_id} | 
[**get_lol_kr_shutdown_law_v1_rating_screen**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_rating_screen) | **Get** /lol-kr-shutdown-law/v1/rating-screen | 
[**get_lol_kr_shutdown_law_v1_status**](PluginLolKrShutdownLawApi.md#get_lol_kr_shutdown_law_v1_status) | **Get** /lol-kr-shutdown-law/v1/status | 
[**post_lol_kr_shutdown_law_v1_rating_screen_acknowledge**](PluginLolKrShutdownLawApi.md#post_lol_kr_shutdown_law_v1_rating_screen_acknowledge) | **Post** /lol-kr-shutdown-law/v1/rating-screen/acknowledge | 



## get_lol_kr_shutdown_law_v1_custom_status

> crate::models::LolKrShutdownLawQueueShutdownStatus get_lol_kr_shutdown_law_v1_custom_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolKrShutdownLawQueueShutdownStatus**](LolKrShutdownLawQueueShutdownStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_kr_shutdown_law_v1_disabled_queues

> Vec<i32> get_lol_kr_shutdown_law_v1_disabled_queues()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_kr_shutdown_law_v1_notification

> crate::models::LolKrShutdownLawShutdownLawNotification get_lol_kr_shutdown_law_v1_notification()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolKrShutdownLawShutdownLawNotification**](LolKrShutdownLawShutdownLawNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_kr_shutdown_law_v1_queue_status_by_queue_id

> crate::models::LolKrShutdownLawQueueShutdownStatus get_lol_kr_shutdown_law_v1_queue_status_by_queue_id(queue_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**queue_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolKrShutdownLawQueueShutdownStatus**](LolKrShutdownLawQueueShutdownStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_kr_shutdown_law_v1_rating_screen

> crate::models::LolKrShutdownLawRatingScreenInfo get_lol_kr_shutdown_law_v1_rating_screen()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolKrShutdownLawRatingScreenInfo**](LolKrShutdownLawRatingScreenInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_kr_shutdown_law_v1_status

> crate::models::LolKrShutdownLawAllQueueShutdownStatus get_lol_kr_shutdown_law_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolKrShutdownLawAllQueueShutdownStatus**](LolKrShutdownLawAllQueueShutdownStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_kr_shutdown_law_v1_rating_screen_acknowledge

> post_lol_kr_shutdown_law_v1_rating_screen_acknowledge()


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

