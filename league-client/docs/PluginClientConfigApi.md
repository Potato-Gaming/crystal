# \PluginClientConfigApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_client_config_v1_config**](PluginClientConfigApi.md#get_client_config_v1_config) | **Get** /client-config/v1/config | 
[**get_client_config_v1_config_by_name**](PluginClientConfigApi.md#get_client_config_v1_config_by_name) | **Get** /client-config/v1/config/{name} | 
[**get_client_config_v1_status_by_type**](PluginClientConfigApi.md#get_client_config_v1_status_by_type) | **Get** /client-config/v1/status/{type} | 
[**get_client_config_v2_config_by_name**](PluginClientConfigApi.md#get_client_config_v2_config_by_name) | **Get** /client-config/v2/config/{name} | 
[**get_client_config_v2_namespace_by_namespace**](PluginClientConfigApi.md#get_client_config_v2_namespace_by_namespace) | **Get** /client-config/v2/namespace/{namespace} | 
[**get_client_config_v2_namespace_by_namespace_player**](PluginClientConfigApi.md#get_client_config_v2_namespace_by_namespace_player) | **Get** /client-config/v2/namespace/{namespace}/player | 
[**get_client_config_v2_namespace_by_namespace_public**](PluginClientConfigApi.md#get_client_config_v2_namespace_by_namespace_public) | **Get** /client-config/v2/namespace/{namespace}/public | 
[**put_client_config_v1_entitlements_token**](PluginClientConfigApi.md#put_client_config_v1_entitlements_token) | **Put** /client-config/v1/entitlements-token | 
[**put_client_config_v1_refresh_config_status**](PluginClientConfigApi.md#put_client_config_v1_refresh_config_status) | **Put** /client-config/v1/refresh-config-status | 
[**put_client_config_v2_namespace_changes**](PluginClientConfigApi.md#put_client_config_v2_namespace_changes) | **Put** /client-config/v2/namespace-changes | 



## get_client_config_v1_config

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v1_config(_type, app, version, patchline, region, namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |
**app** | Option<**String**> |  |  |
**version** | Option<**String**> |  |  |
**patchline** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |
**namespace** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v1_config_by_name

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v1_config_by_name(name, _type, app, version, patchline, region)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**_type** | **String** |  | [required] |
**app** | Option<**String**> |  |  |
**version** | Option<**String**> |  |  |
**patchline** | Option<**String**> |  |  |
**region** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v1_status_by_type

> crate::models::ClientConfigConfigStatus get_client_config_v1_status_by_type(_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**_type** | **String** |  | [required] |

### Return type

[**crate::models::ClientConfigConfigStatus**](ClientConfigConfigStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v2_config_by_name

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v2_config_by_name(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v2_namespace_by_namespace

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v2_namespace_by_namespace(namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v2_namespace_by_namespace_player

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v2_namespace_by_namespace_player(namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client_config_v2_namespace_by_namespace_public

> ::std::collections::HashMap<String, serde_json::Value> get_client_config_v2_namespace_by_namespace_public(namespace)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_client_config_v1_entitlements_token

> put_client_config_v1_entitlements_token(update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update** | [**ClientConfigEntitlementsUpdate**](ClientConfigEntitlementsUpdate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_client_config_v1_refresh_config_status

> put_client_config_v1_refresh_config_status()


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


## put_client_config_v2_namespace_changes

> put_client_config_v2_namespace_changes(namespaces)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespaces** | [**ClientConfigConfigNamespaceUpdate**](ClientConfigConfigNamespaceUpdate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

