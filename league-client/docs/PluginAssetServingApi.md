# \PluginAssetServingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_plugin_assets_by_path**](PluginAssetServingApi.md#get_by_plugin_assets_by_path) | **Get** /{plugin}/assets/{path} | Download a backend asset
[**head_by_plugin_assets_by_path**](PluginAssetServingApi.md#head_by_plugin_assets_by_path) | **Head** /{plugin}/assets/{path} | Download the header for a backend asset



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

