# \PluginManagerInfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_plugin_manager_v2_descriptions**](PluginManagerInfoApi.md#get_plugin_manager_v2_descriptions) | **Get** /plugin-manager/v2/descriptions | Get all plugin descriptions.
[**get_plugin_manager_v2_descriptions_by_plugin**](PluginManagerInfoApi.md#get_plugin_manager_v2_descriptions_by_plugin) | **Get** /plugin-manager/v2/descriptions/{plugin} | Get a plugin description.
[**get_plugin_manager_v3_plugins_manifest**](PluginManagerInfoApi.md#get_plugin_manager_v3_plugins_manifest) | **Get** /plugin-manager/v3/plugins-manifest | Get the plugin manifest.



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

