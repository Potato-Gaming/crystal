# \RiotclientApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_riotclient_affinity**](RiotclientApi.md#delete_riotclient_affinity) | **Delete** /riotclient/affinity | Deletes the current runtime affinity of the application.
[**delete_riotclient_splash**](RiotclientApi.md#delete_riotclient_splash) | **Delete** /riotclient/splash | Hide the splash screen.
[**delete_riotclient_v1_auth_tokens_by_auth_token**](RiotclientApi.md#delete_riotclient_v1_auth_tokens_by_auth_token) | **Delete** /riotclient/v1/auth-tokens/{authToken} | Unregister an existing auth token.
[**get_riotclient_affinity**](RiotclientApi.md#get_riotclient_affinity) | **Get** /riotclient/affinity | Get the current runtime affinity of the application.
[**get_riotclient_app_name**](RiotclientApi.md#get_riotclient_app_name) | **Get** /riotclient/app-name | Application name without file extension
[**get_riotclient_app_port**](RiotclientApi.md#get_riotclient_app_port) | **Get** /riotclient/app-port | Get the TCP port number that the remoting server is listening on.
[**get_riotclient_auth_token**](RiotclientApi.md#get_riotclient_auth_token) | **Get** /riotclient/auth-token | Return the auth token used by the remoting server
[**get_riotclient_command_line_args**](RiotclientApi.md#get_riotclient_command_line_args) | **Get** /riotclient/command-line-args | Get the command line parameters for the application
[**get_riotclient_get_region_locale**](RiotclientApi.md#get_riotclient_get_region_locale) | **Get** /riotclient/get_region_locale | Get the current region and locale.
[**get_riotclient_machine_id**](RiotclientApi.md#get_riotclient_machine_id) | **Get** /riotclient/machine-id | Base64 encoded uuid identifying the user's machine
[**get_riotclient_region_locale**](RiotclientApi.md#get_riotclient_region_locale) | **Get** /riotclient/region-locale | Get the current region and locale.
[**get_riotclient_system_info_v1_basic_info**](RiotclientApi.md#get_riotclient_system_info_v1_basic_info) | **Get** /riotclient/system-info/v1/basic-info | Get basic system information: OS, memory, processor speed, and number of physical cores
[**get_riotclient_trace**](RiotclientApi.md#get_riotclient_trace) | **Get** /riotclient/trace | Retrieves a completed scheduler trace.
[**get_riotclient_ux_crash_count**](RiotclientApi.md#get_riotclient_ux_crash_count) | **Get** /riotclient/ux-crash-count | Returns whether the ux has crashed or not
[**get_riotclient_ux_state**](RiotclientApi.md#get_riotclient_ux_state) | **Get** /riotclient/ux-state | Get the current Ux state.
[**get_riotclient_v1_crash_reporting_environment**](RiotclientApi.md#get_riotclient_v1_crash_reporting_environment) | **Get** /riotclient/v1/crash-reporting/environment | Get the crash reporting environment identifier.
[**get_riotclient_zoom_scale**](RiotclientApi.md#get_riotclient_zoom_scale) | **Get** /riotclient/zoom-scale | Gets the last known posted zoom-scale value.
[**post_riotclient_affinity**](RiotclientApi.md#post_riotclient_affinity) | **Post** /riotclient/affinity | Sets the current runtime affinity of the application.
[**post_riotclient_kill_and_restart_ux**](RiotclientApi.md#post_riotclient_kill_and_restart_ux) | **Post** /riotclient/kill-and-restart-ux | Kills the ux process and restarts it. Used only when the ux process crashes.
[**post_riotclient_kill_ux**](RiotclientApi.md#post_riotclient_kill_ux) | **Post** /riotclient/kill-ux | Kills the ux process.
[**post_riotclient_launch_ux**](RiotclientApi.md#post_riotclient_launch_ux) | **Post** /riotclient/launch-ux | Launches the ux process.
[**post_riotclient_new_args**](RiotclientApi.md#post_riotclient_new_args) | **Post** /riotclient/new-args | Endpoint for passing in new data.
[**post_riotclient_set_region_locale**](RiotclientApi.md#post_riotclient_set_region_locale) | **Post** /riotclient/set_region_locale | Update the region and locale.
[**post_riotclient_show_swagger**](RiotclientApi.md#post_riotclient_show_swagger) | **Post** /riotclient/show-swagger | Open swagger in the default browser.
[**post_riotclient_unload**](RiotclientApi.md#post_riotclient_unload) | **Post** /riotclient/unload | Unloads the UX process
[**post_riotclient_ux_allow_foreground**](RiotclientApi.md#post_riotclient_ux_allow_foreground) | **Post** /riotclient/ux-allow-foreground | Allows the background process to launch the game into the foregound.
[**post_riotclient_ux_flash**](RiotclientApi.md#post_riotclient_ux_flash) | **Post** /riotclient/ux-flash | Flash the ux process' main window and the taskbar/dock icon, if they exist.
[**post_riotclient_ux_minimize**](RiotclientApi.md#post_riotclient_ux_minimize) | **Post** /riotclient/ux-minimize | Minimize the ux process and all its windows if it exists. This does not kill the ux.
[**post_riotclient_ux_show**](RiotclientApi.md#post_riotclient_ux_show) | **Post** /riotclient/ux-show | Shows the ux process if it exists; create and show if it does not.
[**post_riotclient_v1_crash_reporting_logs**](RiotclientApi.md#post_riotclient_v1_crash_reporting_logs) | **Post** /riotclient/v1/crash-reporting/logs | Adds the enclosed log to the app's crash report.
[**post_riotclient_v1_elevation_requests**](RiotclientApi.md#post_riotclient_v1_elevation_requests) | **Post** /riotclient/v1/elevation-requests | 
[**post_riotclient_zoom_scale**](RiotclientApi.md#post_riotclient_zoom_scale) | **Post** /riotclient/zoom-scale | Handles changing the zoom scale value.
[**put_riotclient_region_locale**](RiotclientApi.md#put_riotclient_region_locale) | **Put** /riotclient/region-locale | Update the region and locale.
[**put_riotclient_region_locale_ack**](RiotclientApi.md#put_riotclient_region_locale_ack) | **Put** /riotclient/region-locale/ack | Ux acknowledges the update to the region and locale.
[**put_riotclient_splash**](RiotclientApi.md#put_riotclient_splash) | **Put** /riotclient/splash | Show the splash screen.
[**put_riotclient_ux_load_complete**](RiotclientApi.md#put_riotclient_ux_load_complete) | **Put** /riotclient/ux-load-complete | Ux notification that it has completed loading the main window.
[**put_riotclient_ux_state_ack**](RiotclientApi.md#put_riotclient_ux_state_ack) | **Put** /riotclient/ux-state/ack | Ux acknowledges the update to the Ux state.
[**put_riotclient_v1_auth_tokens_by_auth_token**](RiotclientApi.md#put_riotclient_v1_auth_tokens_by_auth_token) | **Put** /riotclient/v1/auth-tokens/{authToken} | Register an auth token.  This is any alpha-numeric string that will be used as a password with the `riot` user when making requests.
[**put_riotclient_v1_crash_reporting_environment**](RiotclientApi.md#put_riotclient_v1_crash_reporting_environment) | **Put** /riotclient/v1/crash-reporting/environment | Tags the crash with an environment so it can be filtered more easily.
[**put_riotclient_v1_self_update_info**](RiotclientApi.md#put_riotclient_v1_self_update_info) | **Put** /riotclient/v1/self-update-info | 



## delete_riotclient_affinity

> delete_riotclient_affinity()
Deletes the current runtime affinity of the application.

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


## delete_riotclient_splash

> delete_riotclient_splash()
Hide the splash screen.

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


## delete_riotclient_v1_auth_tokens_by_auth_token

> ::std::collections::HashMap<String, serde_json::Value> delete_riotclient_v1_auth_tokens_by_auth_token(auth_token)
Unregister an existing auth token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_token** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_affinity

> ::std::collections::HashMap<String, serde_json::Value> get_riotclient_affinity()
Get the current runtime affinity of the application.

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


## get_riotclient_app_name

> String get_riotclient_app_name()
Application name without file extension

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


## get_riotclient_app_port

> i32 get_riotclient_app_port()
Get the TCP port number that the remoting server is listening on.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_auth_token

> String get_riotclient_auth_token()
Return the auth token used by the remoting server

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


## get_riotclient_command_line_args

> Vec<String> get_riotclient_command_line_args()
Get the command line parameters for the application

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_get_region_locale

> crate::models::RegionLocale get_riotclient_get_region_locale()
Get the current region and locale.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RegionLocale**](RegionLocale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_machine_id

> String get_riotclient_machine_id()
Base64 encoded uuid identifying the user's machine

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


## get_riotclient_region_locale

> crate::models::RegionLocale get_riotclient_region_locale()
Get the current region and locale.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RegionLocale**](RegionLocale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_system_info_v1_basic_info

> crate::models::BasicSystemInfo get_riotclient_system_info_v1_basic_info()
Get basic system information: OS, memory, processor speed, and number of physical cores

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BasicSystemInfo**](basicSystemInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_trace

> ::std::collections::HashMap<String, serde_json::Value> get_riotclient_trace()
Retrieves a completed scheduler trace.

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


## get_riotclient_ux_crash_count

> i32 get_riotclient_ux_crash_count()
Returns whether the ux has crashed or not

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_ux_state

> String get_riotclient_ux_state()
Get the current Ux state.

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


## get_riotclient_v1_crash_reporting_environment

> crate::models::CrashReportingEnvironment get_riotclient_v1_crash_reporting_environment()
Get the crash reporting environment identifier.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::CrashReportingEnvironment**](CrashReportingEnvironment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riotclient_zoom_scale

> f64 get_riotclient_zoom_scale()
Gets the last known posted zoom-scale value.

### Parameters

This endpoint does not need any parameter.

### Return type

**f64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_affinity

> post_riotclient_affinity(new_affinity)
Sets the current runtime affinity of the application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_affinity** | **String** | The new affinity to use. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_kill_and_restart_ux

> post_riotclient_kill_and_restart_ux()
Kills the ux process and restarts it. Used only when the ux process crashes.

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


## post_riotclient_kill_ux

> post_riotclient_kill_ux()
Kills the ux process.

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


## post_riotclient_launch_ux

> post_riotclient_launch_ux()
Launches the ux process.

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


## post_riotclient_new_args

> post_riotclient_new_args(UNKNOWN_BASE_TYPE)
Endpoint for passing in new data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**UNKNOWN_BASE_TYPE** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_set_region_locale

> post_riotclient_set_region_locale(region, locale)
Update the region and locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region** | **String** | Name of the region. | [required] |
**locale** | **String** | Name of the locale. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_show_swagger

> post_riotclient_show_swagger()
Open swagger in the default browser.

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


## post_riotclient_unload

> post_riotclient_unload()
Unloads the UX process

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


## post_riotclient_ux_allow_foreground

> post_riotclient_ux_allow_foreground()
Allows the background process to launch the game into the foregound.

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


## post_riotclient_ux_flash

> post_riotclient_ux_flash()
Flash the ux process' main window and the taskbar/dock icon, if they exist.

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


## post_riotclient_ux_minimize

> post_riotclient_ux_minimize()
Minimize the ux process and all its windows if it exists. This does not kill the ux.

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


## post_riotclient_ux_show

> post_riotclient_ux_show()
Shows the ux process if it exists; create and show if it does not.

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


## post_riotclient_v1_crash_reporting_logs

> post_riotclient_v1_crash_reporting_logs(log_file_path)
Adds the enclosed log to the app's crash report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_file_path** | **String** | Full path to the log file. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_v1_elevation_requests

> post_riotclient_v1_elevation_requests(request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**ElevationRequest**](ElevationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riotclient_zoom_scale

> post_riotclient_zoom_scale(new_zoom_scale)
Handles changing the zoom scale value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_zoom_scale** | **f64** | The new value of the zoom scale. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_region_locale

> put_riotclient_region_locale(data)
Update the region and locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**RegionLocale**](RegionLocale.md) | Region and locale resource. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_region_locale_ack

> put_riotclient_region_locale_ack(request_id)
Ux acknowledges the update to the region and locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **i32** | The region and locale change requestId that is being acknowledged. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_splash

> put_riotclient_splash()
Show the splash screen.

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


## put_riotclient_ux_load_complete

> put_riotclient_ux_load_complete()
Ux notification that it has completed loading the main window.

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


## put_riotclient_ux_state_ack

> put_riotclient_ux_state_ack(request_id)
Ux acknowledges the update to the Ux state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **i32** | The ux change requestId that is being acknowledged. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_v1_auth_tokens_by_auth_token

> ::std::collections::HashMap<String, serde_json::Value> put_riotclient_v1_auth_tokens_by_auth_token(auth_token)
Register an auth token.  This is any alpha-numeric string that will be used as a password with the `riot` user when making requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_token** | **String** | Authentication token to add. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_v1_crash_reporting_environment

> put_riotclient_v1_crash_reporting_environment(environment)
Tags the crash with an environment so it can be filtered more easily.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**environment** | [**CrashReportingEnvironment**](CrashReportingEnvironment.md) | The environment identifier that should be added to the crash report. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_riotclient_v1_self_update_info

> put_riotclient_v1_self_update_info(url, uuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |  | [required] |
**uuid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

