# \TracingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tracing_v1_trace_time_series_event_by_event_name**](TracingApi.md#delete_tracing_v1_trace_time_series_event_by_event_name) | **Delete** /tracing/v1/trace/time-series-event/{eventName} | Record the ending of a time series event.
[**post_tracing_v1_trace_event**](TracingApi.md#post_tracing_v1_trace_event) | **Post** /tracing/v1/trace/event | Record a tracing event.
[**post_tracing_v1_trace_module**](TracingApi.md#post_tracing_v1_trace_module) | **Post** /tracing/v1/trace/module | Record a module description.
[**post_tracing_v1_trace_phase_begin**](TracingApi.md#post_tracing_v1_trace_phase_begin) | **Post** /tracing/v1/trace/phase/begin | Record a tracing phase beginning.
[**post_tracing_v1_trace_phase_end**](TracingApi.md#post_tracing_v1_trace_phase_end) | **Post** /tracing/v1/trace/phase/end | Record a tracing phase ending.
[**post_tracing_v1_trace_time_series_event_by_event_name**](TracingApi.md#post_tracing_v1_trace_time_series_event_by_event_name) | **Post** /tracing/v1/trace/time-series-event/{eventName} | Record the beginning of a time series event.
[**post_tracing_v1_trace_time_series_event_by_event_name_marker_by_marker_name**](TracingApi.md#post_tracing_v1_trace_time_series_event_by_event_name_marker_by_marker_name) | **Post** /tracing/v1/trace/time-series-event/{eventName}/marker/{markerName} | Record a marker within a time series event.



## delete_tracing_v1_trace_time_series_event_by_event_name

> delete_tracing_v1_trace_time_series_event_by_event_name(event_name, when, suffix)
Record the ending of a time series event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | Name of the event | [required] |
**when** | **i64** | When the event occurred | [required] |
**suffix** | Option<**String**> | Optional eventName suffix to apply that mutates the event name to indicate the outcome |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_event

> post_tracing_v1_trace_event(tracing_event)
Record a tracing event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tracing_event** | [**TracingEventV1**](TracingEventV1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_module

> post_tracing_v1_trace_module(module)
Record a module description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module** | [**TracingModuleV1**](TracingModuleV1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_phase_begin

> post_tracing_v1_trace_phase_begin(phase_begin)
Record a tracing phase beginning.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phase_begin** | [**TracingPhaseBeginV1**](TracingPhaseBeginV1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_phase_end

> post_tracing_v1_trace_phase_end(phase_end)
Record a tracing phase ending.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**phase_end** | [**TracingPhaseEndV1**](TracingPhaseEndV1.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_time_series_event_by_event_name

> post_tracing_v1_trace_time_series_event_by_event_name(event_name, when)
Record the beginning of a time series event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | Name of the event | [required] |
**when** | **i64** | When the event occurred | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tracing_v1_trace_time_series_event_by_event_name_marker_by_marker_name

> post_tracing_v1_trace_time_series_event_by_event_name_marker_by_marker_name(event_name, marker_name, when)
Record a marker within a time series event.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | Plugin name | [required] |
**marker_name** | **String** | Name of the marker within the event | [required] |
**when** | **i64** | When the marker event occurred | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

