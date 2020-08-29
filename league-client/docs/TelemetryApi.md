# \TelemetryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_telemetry_v1_application_start_time**](TelemetryApi.md#get_telemetry_v1_application_start_time) | **Get** /telemetry/v1/application-start-time | Gets the millisecond UNIX timestamp of when the application was started.
[**post_telemetry_v1_common_data_by_key**](TelemetryApi.md#post_telemetry_v1_common_data_by_key) | **Post** /telemetry/v1/common-data/{key} | Adds/updates a common data key and value to be sent with every subsequent event.
[**post_telemetry_v1_events_by_event_type**](TelemetryApi.md#post_telemetry_v1_events_by_event_type) | **Post** /telemetry/v1/events/{eventType} | Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. All events will have their eventType prefixed with \"\"
[**post_telemetry_v1_events_once_by_event_type**](TelemetryApi.md#post_telemetry_v1_events_once_by_event_type) | **Post** /telemetry/v1/events-once/{eventType} | Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks that will be sent only once during this client executable run regardless of any javascript frontend restarts. All events will have their eventType prefixed with \"\"
[**post_telemetry_v1_events_with_perf_info_by_event_type**](TelemetryApi.md#post_telemetry_v1_events_with_perf_info_by_event_type) | **Post** /telemetry/v1/events-with-perf-info/{eventType} | Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. This will include current performance information along with the passed in data. Each call will record the performance counters then reset them for use in the next call. All events will have their eventType prefixed with \"\"
[**post_telemetry_v3_events_by_event_type**](TelemetryApi.md#post_telemetry_v3_events_by_event_type) | **Post** /telemetry/v3/events/{eventType} | Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API. 
[**post_telemetry_v3_events_once_by_event_type**](TelemetryApi.md#post_telemetry_v3_events_once_by_event_type) | **Post** /telemetry/v3/events-once/{eventType} | Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API that will be sent only once during this client executable run regardless of any javascript frontend restarts. All events will have their eventType prefixed with \"\"



## get_telemetry_v1_application_start_time

> i64 get_telemetry_v1_application_start_time()
Gets the millisecond UNIX timestamp of when the application was started.

### Parameters

This endpoint does not need any parameter.

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v1_common_data_by_key

> post_telemetry_v1_common_data_by_key(key, value)
Adds/updates a common data key and value to be sent with every subsequent event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** | The name of the common data key | [required] |
**value** | **String** | The value of the common data key | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v1_events_by_event_type

> post_telemetry_v1_events_by_event_type(event_type, event_data)
Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. All events will have their eventType prefixed with \"\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** | The name of the event type | [required] |
**event_data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A map of event data | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v1_events_once_by_event_type

> post_telemetry_v1_events_once_by_event_type(event_type, once_tag, event_data)
Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks that will be sent only once during this client executable run regardless of any javascript frontend restarts. All events will have their eventType prefixed with \"\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** | The name of the event type | [required] |
**once_tag** | **String** | A unique tag used to ensure this event is sent only once | [required] |
**event_data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A map of event data | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v1_events_with_perf_info_by_event_type

> post_telemetry_v1_events_with_perf_info_by_event_type(event_type, event_data)
Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks. This will include current performance information along with the passed in data. Each call will record the performance counters then reset them for use in the next call. All events will have their eventType prefixed with \"\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** | The name of the event type | [required] |
**event_data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A map of event data | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v3_events_by_event_type

> post_telemetry_v3_events_by_event_type(event_type, event_data)
Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** | The name of the event type | [required] |
**event_data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A map of event data | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_telemetry_v3_events_once_by_event_type

> post_telemetry_v3_events_once_by_event_type(event_type, once_tag, event_data)
Adds a new event to be sent to Dradis and/or other analytics/monitoring data sinks using the Riot Data API that will be sent only once during this client executable run regardless of any javascript frontend restarts. All events will have their eventType prefixed with \"\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_type** | **String** | The name of the event type | [required] |
**once_tag** | **String** | A unique tag used to ensure this event is sent only once | [required] |
**event_data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | A map of event data | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

