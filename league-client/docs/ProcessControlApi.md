# \ProcessControlApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_process_control_v1_process**](ProcessControlApi.md#get_process_control_v1_process) | **Get** /process-control/v1/process | Returns information about the process-control.
[**post_process_control_v1_process_quit**](ProcessControlApi.md#post_process_control_v1_process_quit) | **Post** /process-control/v1/process/quit | Quits the application.
[**post_process_control_v1_process_restart**](ProcessControlApi.md#post_process_control_v1_process_restart) | **Post** /process-control/v1/process/restart | Restarts the application.  Does nothing if there is already a waiting delayed restart.  Optionally accepts specific version to restart.
[**post_process_control_v1_process_restart_to_repair**](ProcessControlApi.md#post_process_control_v1_process_restart_to_repair) | **Post** /process-control/v1/process/restart-to-repair | Restarts the application in order to perform a full repair (including self repair).



## get_process_control_v1_process

> crate::models::ProcessControlProcess get_process_control_v1_process()
Returns information about the process-control.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ProcessControlProcess**](ProcessControlProcess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_process_control_v1_process_quit

> post_process_control_v1_process_quit()
Quits the application.

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


## post_process_control_v1_process_restart

> post_process_control_v1_process_restart(delay_seconds, restart_version)
Restarts the application.  Does nothing if there is already a waiting delayed restart.  Optionally accepts specific version to restart.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delay_seconds** | **i32** |  | [required] |
**restart_version** | Option<**i32**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_process_control_v1_process_restart_to_repair

> post_process_control_v1_process_restart_to_repair()
Restarts the application in order to perform a full repair (including self repair).

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

