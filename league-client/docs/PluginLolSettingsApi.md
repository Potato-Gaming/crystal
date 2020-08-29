# \PluginLolSettingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_settings_v1_account_by_category**](PluginLolSettingsApi.md#get_lol_settings_v1_account_by_category) | **Get** /lol-settings/v1/account/{category} | 
[**get_lol_settings_v1_account_didreset**](PluginLolSettingsApi.md#get_lol_settings_v1_account_didreset) | **Get** /lol-settings/v1/account/didreset | 
[**get_lol_settings_v1_local_by_category**](PluginLolSettingsApi.md#get_lol_settings_v1_local_by_category) | **Get** /lol-settings/v1/local/{category} | 
[**get_lol_settings_v2_account_by_pp_type_by_category**](PluginLolSettingsApi.md#get_lol_settings_v2_account_by_pp_type_by_category) | **Get** /lol-settings/v2/account/{ppType}/{category} | 
[**get_lol_settings_v2_didreset_by_pp_type**](PluginLolSettingsApi.md#get_lol_settings_v2_didreset_by_pp_type) | **Get** /lol-settings/v2/didreset/{ppType} | 
[**get_lol_settings_v2_local_by_category**](PluginLolSettingsApi.md#get_lol_settings_v2_local_by_category) | **Get** /lol-settings/v2/local/{category} | 
[**get_lol_settings_v2_ready**](PluginLolSettingsApi.md#get_lol_settings_v2_ready) | **Get** /lol-settings/v2/ready | 
[**patch_lol_settings_v1_account_by_category**](PluginLolSettingsApi.md#patch_lol_settings_v1_account_by_category) | **Patch** /lol-settings/v1/account/{category} | 
[**patch_lol_settings_v1_local_by_category**](PluginLolSettingsApi.md#patch_lol_settings_v1_local_by_category) | **Patch** /lol-settings/v1/local/{category} | 
[**patch_lol_settings_v2_account_by_pp_type_by_category**](PluginLolSettingsApi.md#patch_lol_settings_v2_account_by_pp_type_by_category) | **Patch** /lol-settings/v2/account/{ppType}/{category} | 
[**patch_lol_settings_v2_local_by_category**](PluginLolSettingsApi.md#patch_lol_settings_v2_local_by_category) | **Patch** /lol-settings/v2/local/{category} | 
[**post_lol_settings_v1_account_save**](PluginLolSettingsApi.md#post_lol_settings_v1_account_save) | **Post** /lol-settings/v1/account/save | 
[**put_lol_settings_v1_account_by_category**](PluginLolSettingsApi.md#put_lol_settings_v1_account_by_category) | **Put** /lol-settings/v1/account/{category} | 
[**put_lol_settings_v2_account_by_pp_type_by_category**](PluginLolSettingsApi.md#put_lol_settings_v2_account_by_pp_type_by_category) | **Put** /lol-settings/v2/account/{ppType}/{category} | 



## get_lol_settings_v1_account_by_category

> ::std::collections::HashMap<String, serde_json::Value> get_lol_settings_v1_account_by_category(category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_settings_v1_account_didreset

> bool get_lol_settings_v1_account_didreset()


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


## get_lol_settings_v1_local_by_category

> ::std::collections::HashMap<String, serde_json::Value> get_lol_settings_v1_local_by_category(category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_settings_v2_account_by_pp_type_by_category

> ::std::collections::HashMap<String, serde_json::Value> get_lol_settings_v2_account_by_pp_type_by_category(pp_type, category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pp_type** | **String** |  | [required] |
**category** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_settings_v2_didreset_by_pp_type

> bool get_lol_settings_v2_didreset_by_pp_type(pp_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pp_type** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_settings_v2_local_by_category

> ::std::collections::HashMap<String, serde_json::Value> get_lol_settings_v2_local_by_category(category)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_settings_v2_ready

> bool get_lol_settings_v2_ready()


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


## patch_lol_settings_v1_account_by_category

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_settings_v1_account_by_category(category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_settings_v1_local_by_category

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_settings_v1_local_by_category(category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_settings_v2_account_by_pp_type_by_category

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_settings_v2_account_by_pp_type_by_category(pp_type, category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pp_type** | **String** |  | [required] |
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_settings_v2_local_by_category

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_settings_v2_local_by_category(category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_settings_v1_account_save

> post_lol_settings_v1_account_save()


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


## put_lol_settings_v1_account_by_category

> ::std::collections::HashMap<String, serde_json::Value> put_lol_settings_v1_account_by_category(category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_settings_v2_account_by_pp_type_by_category

> ::std::collections::HashMap<String, serde_json::Value> put_lol_settings_v2_account_by_pp_type_by_category(pp_type, category, settings_resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pp_type** | **String** |  | [required] |
**category** | **String** |  | [required] |
**settings_resource** | [**LolSettingsSettingCategory**](LolSettingsSettingCategory.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

