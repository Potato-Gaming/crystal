# \PluginManagerDiagnosticsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plugin_manager_v1_external_plugins_availability**](PluginManagerDiagnosticsApi.md#get_plugin_manager_v1_external_plugins_availability) | **Get** /plugin-manager/v1/external-plugins/availability | Get the status of the external plugin connection.
[**get_plugin_manager_v1_status**](PluginManagerDiagnosticsApi.md#get_plugin_manager_v1_status) | **Get** /plugin-manager/v1/status | Get the status of the plugin manager.
[**get_plugin_manager_v2_plugins**](PluginManagerDiagnosticsApi.md#get_plugin_manager_v2_plugins) | **Get** /plugin-manager/v2/plugins | Get diagnostic information for all plugins.
[**get_plugin_manager_v2_plugins_by_plugin**](PluginManagerDiagnosticsApi.md#get_plugin_manager_v2_plugins_by_plugin) | **Get** /plugin-manager/v2/plugins/{plugin} | Get diagnostic information for a single plugin.



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

