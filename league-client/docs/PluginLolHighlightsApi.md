# \PluginLolHighlightsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_highlights_v1_highlights_by_id**](PluginLolHighlightsApi.md#delete_lol_highlights_v1_highlights_by_id) | **Delete** /lol-highlights/v1/highlights/{id} | 
[**get_lol_highlights_v1_config**](PluginLolHighlightsApi.md#get_lol_highlights_v1_config) | **Get** /lol-highlights/v1/config | 
[**get_lol_highlights_v1_highlights**](PluginLolHighlightsApi.md#get_lol_highlights_v1_highlights) | **Get** /lol-highlights/v1/highlights | 
[**get_lol_highlights_v1_highlights_by_id**](PluginLolHighlightsApi.md#get_lol_highlights_v1_highlights_by_id) | **Get** /lol-highlights/v1/highlights/{id} | 
[**get_lol_highlights_v1_highlights_folder_path**](PluginLolHighlightsApi.md#get_lol_highlights_v1_highlights_folder_path) | **Get** /lol-highlights/v1/highlights-folder-path | 
[**get_lol_highlights_v1_highlights_folder_path_default**](PluginLolHighlightsApi.md#get_lol_highlights_v1_highlights_folder_path_default) | **Get** /lol-highlights/v1/highlights-folder-path/default | 
[**post_lol_highlights_v1_file_browser_by_highlight_id**](PluginLolHighlightsApi.md#post_lol_highlights_v1_file_browser_by_highlight_id) | **Post** /lol-highlights/v1/file-browser/{highlightId} | 
[**post_lol_highlights_v1_highlights**](PluginLolHighlightsApi.md#post_lol_highlights_v1_highlights) | **Post** /lol-highlights/v1/highlights | 
[**put_lol_highlights_v1_highlights_by_id**](PluginLolHighlightsApi.md#put_lol_highlights_v1_highlights_by_id) | **Put** /lol-highlights/v1/highlights/{id} | 



## delete_lol_highlights_v1_highlights_by_id

> crate::models::LolHighlightsHighlight delete_lol_highlights_v1_highlights_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolHighlightsHighlight**](LolHighlightsHighlight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_highlights_v1_config

> crate::models::LolHighlightsHighlightsConfig get_lol_highlights_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHighlightsHighlightsConfig**](LolHighlightsHighlightsConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_highlights_v1_highlights

> Vec<crate::models::LolHighlightsHighlight> get_lol_highlights_v1_highlights()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolHighlightsHighlight>**](LolHighlightsHighlight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_highlights_v1_highlights_by_id

> crate::models::LolHighlightsHighlight get_lol_highlights_v1_highlights_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolHighlightsHighlight**](LolHighlightsHighlight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_highlights_v1_highlights_folder_path

> String get_lol_highlights_v1_highlights_folder_path()


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


## get_lol_highlights_v1_highlights_folder_path_default

> String get_lol_highlights_v1_highlights_folder_path_default()


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


## post_lol_highlights_v1_file_browser_by_highlight_id

> ::std::collections::HashMap<String, serde_json::Value> post_lol_highlights_v1_file_browser_by_highlight_id(highlight_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**highlight_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_highlights_v1_highlights

> Vec<crate::models::LolHighlightsHighlight> post_lol_highlights_v1_highlights()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolHighlightsHighlight>**](LolHighlightsHighlight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_highlights_v1_highlights_by_id

> crate::models::LolHighlightsHighlight put_lol_highlights_v1_highlights_by_id(id, highlight)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**highlight** | [**LolHighlightsHighlight**](LolHighlightsHighlight.md) |  | [required] |

### Return type

[**crate::models::LolHighlightsHighlight**](LolHighlightsHighlight.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

