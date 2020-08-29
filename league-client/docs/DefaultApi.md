# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**logging_get_entries**](DefaultApi.md#logging_get_entries) | **Post** /LoggingGetEntries | Gets all buffered log entries since the last call.
[**logging_metrics**](DefaultApi.md#logging_metrics) | **Post** /LoggingMetrics | Returns all metrics
[**logging_metrics_metadata**](DefaultApi.md#logging_metrics_metadata) | **Post** /LoggingMetricsMetadata | Returns metadata for all metrics
[**logging_start**](DefaultApi.md#logging_start) | **Post** /LoggingStart | Initializes the logging system.
[**logging_stop**](DefaultApi.md#logging_stop) | **Post** /LoggingStop | Finalizes the logging system.
[**memory_pools**](DefaultApi.md#memory_pools) | **Post** /MemoryPools | Returns current pool usage



## logging_get_entries

> Vec<crate::models::LogEvent> logging_get_entries()
Gets all buffered log entries since the last call.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LogEvent>**](LogEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logging_metrics

> ::std::collections::HashMap<String, serde_json::Value> logging_metrics()
Returns all metrics

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


## logging_metrics_metadata

> ::std::collections::HashMap<String, serde_json::Value> logging_metrics_metadata()
Returns metadata for all metrics

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


## logging_start

> logging_start(buffered, severity)
Initializes the logging system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**buffered** | Option<**bool**> | Specifies whether logs will be buffered for LoggingGetEntries to work |  |
**severity** | Option<**String**> | Minimum severity level to fire a log event |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logging_stop

> logging_stop()
Finalizes the logging system.

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


## memory_pools

> ::std::collections::HashMap<String, serde_json::Value> memory_pools(context_name)
Returns current pool usage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_name** | Option<**String**> | Name of the context to find (optional) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

