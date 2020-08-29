# \PluginLolPlatformConfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_platform_config_v1_initial_configuration_complete**](PluginLolPlatformConfigApi.md#get_lol_platform_config_v1_initial_configuration_complete) | **Get** /lol-platform-config/v1/initial-configuration-complete | 
[**get_lol_platform_config_v1_namespaces**](PluginLolPlatformConfigApi.md#get_lol_platform_config_v1_namespaces) | **Get** /lol-platform-config/v1/namespaces | 
[**get_lol_platform_config_v1_namespaces_by_ns**](PluginLolPlatformConfigApi.md#get_lol_platform_config_v1_namespaces_by_ns) | **Get** /lol-platform-config/v1/namespaces/{ns} | 
[**get_lol_platform_config_v1_namespaces_by_ns_by_key**](PluginLolPlatformConfigApi.md#get_lol_platform_config_v1_namespaces_by_ns_by_key) | **Get** /lol-platform-config/v1/namespaces/{ns}/{key} | 



## get_lol_platform_config_v1_initial_configuration_complete

> bool get_lol_platform_config_v1_initial_configuration_complete()


### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_platform_config_v1_namespaces

> ::std::collections::HashMap<String, serde_json::Value> get_lol_platform_config_v1_namespaces()


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


## get_lol_platform_config_v1_namespaces_by_ns

> ::std::collections::HashMap<String, serde_json::Value> get_lol_platform_config_v1_namespaces_by_ns(ns)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ns** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_platform_config_v1_namespaces_by_ns_by_key

> ::std::collections::HashMap<String, serde_json::Value> get_lol_platform_config_v1_namespaces_by_ns_by_key(ns, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ns** | **String** |  | [required] |
**key** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

