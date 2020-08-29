# \PerformanceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_performance_v1_memory**](PerformanceApi.md#get_performance_v1_memory) | **Get** /performance/v1/memory | Returns process memory status
[**get_performance_v1_report**](PerformanceApi.md#get_performance_v1_report) | **Get** /performance/v1/report | Returns the various performance information for the cef processes
[**get_performance_v1_system_info**](PerformanceApi.md#get_performance_v1_system_info) | **Get** /performance/v1/system-info | Returns hardware and software specs for the machine the client is running on.
[**post_performance_v1_process_by_process_id**](PerformanceApi.md#post_performance_v1_process_by_process_id) | **Post** /performance/v1/process/{processId} | Registers the process and includes it with the performance information.
[**post_performance_v1_report_restart**](PerformanceApi.md#post_performance_v1_report_restart) | **Post** /performance/v1/report/restart | Restarts the CPU timing information and returns the results from PerfReportProcesses



## get_performance_v1_memory

> ::std::collections::HashMap<String, serde_json::Value> get_performance_v1_memory()
Returns process memory status

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


## get_performance_v1_report

> ::std::collections::HashMap<String, serde_json::Value> get_performance_v1_report()
Returns the various performance information for the cef processes

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


## get_performance_v1_system_info

> ::std::collections::HashMap<String, serde_json::Value> get_performance_v1_system_info(full)
Returns hardware and software specs for the machine the client is running on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full** | Option<**i32**> | Returns all available system information |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_performance_v1_process_by_process_id

> post_performance_v1_process_by_process_id(process_id)
Registers the process and includes it with the performance information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**process_id** | **i32** | Id of the process to track performance information. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_performance_v1_report_restart

> ::std::collections::HashMap<String, serde_json::Value> post_performance_v1_report_restart(sample_length, sample_count)
Restarts the CPU timing information and returns the results from PerfReportProcesses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sample_length** | Option<**i32**> | Time in seconds for each CPU timing sample. |  |
**sample_count** | Option<**i32**> | Number of samples to record. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

