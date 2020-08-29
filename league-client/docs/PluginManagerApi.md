# \PluginManagerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_plugin_assets_by_path**](PluginManagerApi.md#get_by_plugin_assets_by_path) | **Get** /{plugin}/assets/{path} | Download a backend asset
[**get_plugin_manager_v1_external_plugins_availability**](PluginManagerApi.md#get_plugin_manager_v1_external_plugins_availability) | **Get** /plugin-manager/v1/external-plugins/availability | Get the status of the external plugin connection.
[**get_plugin_manager_v1_status**](PluginManagerApi.md#get_plugin_manager_v1_status) | **Get** /plugin-manager/v1/status | Get the status of the plugin manager.
[**get_plugin_manager_v2_descriptions**](PluginManagerApi.md#get_plugin_manager_v2_descriptions) | **Get** /plugin-manager/v2/descriptions | Get all plugin descriptions.
[**get_plugin_manager_v2_descriptions_by_plugin**](PluginManagerApi.md#get_plugin_manager_v2_descriptions_by_plugin) | **Get** /plugin-manager/v2/descriptions/{plugin} | Get a plugin description.
[**get_plugin_manager_v2_plugins**](PluginManagerApi.md#get_plugin_manager_v2_plugins) | **Get** /plugin-manager/v2/plugins | Get diagnostic information for all plugins.
[**get_plugin_manager_v2_plugins_by_plugin**](PluginManagerApi.md#get_plugin_manager_v2_plugins_by_plugin) | **Get** /plugin-manager/v2/plugins/{plugin} | Get diagnostic information for a single plugin.
[**get_plugin_manager_v3_plugins_manifest**](PluginManagerApi.md#get_plugin_manager_v3_plugins_manifest) | **Get** /plugin-manager/v3/plugins-manifest | Get the plugin manifest.
[**head_by_plugin_assets_by_path**](PluginManagerApi.md#head_by_plugin_assets_by_path) | **Head** /{plugin}/assets/{path} | Download the header for a backend asset



## get_by_plugin_assets_by_path

> ::std::collections::HashMap<String, serde_json::Value> get_by_plugin_assets_by_path(plugin, path, if_none_match)
Download a backend asset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** | Plugin name to serve from | [required] |
**path** | **String** | Path to the asset to serve | [required] |
**if_none_match** | Option<**String**> | optional ETag of the asset that the caller has cached |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v1_external_plugins_availability

> crate::models::ExternalPluginsResource get_plugin_manager_v1_external_plugins_availability()
Get the status of the external plugin connection.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ExternalPluginsResource**](ExternalPluginsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v1_status

> crate::models::PluginManagerResource get_plugin_manager_v1_status()
Get the status of the plugin manager.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PluginManagerResource**](PluginManagerResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v2_descriptions

> Vec<crate::models::PluginDescriptionResource> get_plugin_manager_v2_descriptions()
Get all plugin descriptions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PluginDescriptionResource>**](PluginDescriptionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v2_descriptions_by_plugin

> crate::models::PluginDescriptionResource get_plugin_manager_v2_descriptions_by_plugin(plugin)
Get a plugin description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** | Plugin name | [required] |

### Return type

[**crate::models::PluginDescriptionResource**](PluginDescriptionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v2_plugins

> Vec<crate::models::PluginResource> get_plugin_manager_v2_plugins()
Get diagnostic information for all plugins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PluginResource>**](PluginResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v2_plugins_by_plugin

> crate::models::PluginResource get_plugin_manager_v2_plugins_by_plugin(plugin)
Get diagnostic information for a single plugin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** | Plugin name | [required] |

### Return type

[**crate::models::PluginResource**](PluginResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plugin_manager_v3_plugins_manifest

> ::std::collections::HashMap<String, serde_json::Value> get_plugin_manager_v3_plugins_manifest()
Get the plugin manifest.

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


## head_by_plugin_assets_by_path

> ::std::collections::HashMap<String, serde_json::Value> head_by_plugin_assets_by_path(plugin, path, if_none_match)
Download the header for a backend asset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plugin** | **String** | Plugin name to serve from | [required] |
**path** | **String** | Path to the asset to serve | [required] |
**if_none_match** | Option<**String**> | optional ETag of the asset that the caller has cached |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

