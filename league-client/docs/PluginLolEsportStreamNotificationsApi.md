# \PluginLolEsportStreamNotificationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_esport_stream_notifications_v1_live_streams**](PluginLolEsportStreamNotificationsApi.md#get_lol_esport_stream_notifications_v1_live_streams) | **Get** /lol-esport-stream-notifications/v1/live-streams | 
[**get_lol_esport_stream_notifications_v1_stream_url**](PluginLolEsportStreamNotificationsApi.md#get_lol_esport_stream_notifications_v1_stream_url) | **Get** /lol-esport-stream-notifications/v1/stream-url | 
[**post_lol_esport_stream_notifications_v1_send_stats**](PluginLolEsportStreamNotificationsApi.md#post_lol_esport_stream_notifications_v1_send_stats) | **Post** /lol-esport-stream-notifications/v1/send-stats | 



## get_lol_esport_stream_notifications_v1_live_streams

> crate::models::LolEsportStreamNotificationsESportsLiveStreams get_lol_esport_stream_notifications_v1_live_streams()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEsportStreamNotificationsESportsLiveStreams**](LolEsportStreamNotificationsESportsLiveStreams.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_esport_stream_notifications_v1_stream_url

> String get_lol_esport_stream_notifications_v1_stream_url()


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


## post_lol_esport_stream_notifications_v1_send_stats

> post_lol_esport_stream_notifications_v1_send_stats(event_type, match_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** |  | [required] |
**match_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

