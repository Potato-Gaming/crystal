# \DataStoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_data_store_v1_install_dir**](DataStoreApi.md#get_data_store_v1_install_dir) | **Get** /data-store/v1/install-dir | Gets the current install directory (used internally.)
[**get_data_store_v1_install_settings_by_path**](DataStoreApi.md#get_data_store_v1_install_settings_by_path) | **Get** /data-store/v1/install-settings/{path} | Get the data for the specified key from the install settings.
[**get_data_store_v1_system_settings_by_path**](DataStoreApi.md#get_data_store_v1_system_settings_by_path) | **Get** /data-store/v1/system-settings/{path} | Get the setting for the specified key.
[**post_data_store_v1_install_settings_by_path**](DataStoreApi.md#post_data_store_v1_install_settings_by_path) | **Post** /data-store/v1/install-settings/{path} | Set the data for the specified key from the install settings.



## get_data_store_v1_install_dir

> String get_data_store_v1_install_dir()
Gets the current install directory (used internally.)

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


## get_data_store_v1_install_settings_by_path

> ::std::collections::HashMap<String, serde_json::Value> get_data_store_v1_install_settings_by_path(path)
Get the data for the specified key from the install settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The path to the settings key | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_data_store_v1_system_settings_by_path

> ::std::collections::HashMap<String, serde_json::Value> get_data_store_v1_system_settings_by_path(path)
Get the setting for the specified key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The path to the settings key | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_data_store_v1_install_settings_by_path

> post_data_store_v1_install_settings_by_path(path, data)
Set the data for the specified key from the install settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** | The path to the settings key | [required] |
**data** | **serde_json::Value** | The data to assign to the key | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

