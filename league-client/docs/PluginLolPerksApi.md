# \PluginLolPerksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_perks_v1_pages**](PluginLolPerksApi.md#delete_lol_perks_v1_pages) | **Delete** /lol-perks/v1/pages | 
[**delete_lol_perks_v1_pages_by_id**](PluginLolPerksApi.md#delete_lol_perks_v1_pages_by_id) | **Delete** /lol-perks/v1/pages/{id} | 
[**delete_lol_perks_v1_pages_by_id_auto_modified_selections**](PluginLolPerksApi.md#delete_lol_perks_v1_pages_by_id_auto_modified_selections) | **Delete** /lol-perks/v1/pages/{id}/auto-modified-selections | 
[**get_lol_perks_v1_currentpage**](PluginLolPerksApi.md#get_lol_perks_v1_currentpage) | **Get** /lol-perks/v1/currentpage | 
[**get_lol_perks_v1_customizationlimits**](PluginLolPerksApi.md#get_lol_perks_v1_customizationlimits) | **Get** /lol-perks/v1/customizationlimits | 
[**get_lol_perks_v1_inventory**](PluginLolPerksApi.md#get_lol_perks_v1_inventory) | **Get** /lol-perks/v1/inventory | 
[**get_lol_perks_v1_pages**](PluginLolPerksApi.md#get_lol_perks_v1_pages) | **Get** /lol-perks/v1/pages | 
[**get_lol_perks_v1_pages_by_id**](PluginLolPerksApi.md#get_lol_perks_v1_pages_by_id) | **Get** /lol-perks/v1/pages/{id} | 
[**get_lol_perks_v1_perks**](PluginLolPerksApi.md#get_lol_perks_v1_perks) | **Get** /lol-perks/v1/perks | 
[**get_lol_perks_v1_perks_disabled**](PluginLolPerksApi.md#get_lol_perks_v1_perks_disabled) | **Get** /lol-perks/v1/perks/disabled | 
[**get_lol_perks_v1_perks_gameplay_updated**](PluginLolPerksApi.md#get_lol_perks_v1_perks_gameplay_updated) | **Get** /lol-perks/v1/perks/gameplay-updated | 
[**get_lol_perks_v1_schema_version**](PluginLolPerksApi.md#get_lol_perks_v1_schema_version) | **Get** /lol-perks/v1/schema-version | 
[**get_lol_perks_v1_servicesettings**](PluginLolPerksApi.md#get_lol_perks_v1_servicesettings) | **Get** /lol-perks/v1/servicesettings | 
[**get_lol_perks_v1_settings**](PluginLolPerksApi.md#get_lol_perks_v1_settings) | **Get** /lol-perks/v1/settings | 
[**get_lol_perks_v1_show_auto_modified_pages_notification**](PluginLolPerksApi.md#get_lol_perks_v1_show_auto_modified_pages_notification) | **Get** /lol-perks/v1/show-auto-modified-pages-notification | 
[**get_lol_perks_v1_styles**](PluginLolPerksApi.md#get_lol_perks_v1_styles) | **Get** /lol-perks/v1/styles | 
[**post_lol_perks_v1_pages**](PluginLolPerksApi.md#post_lol_perks_v1_pages) | **Post** /lol-perks/v1/pages | 
[**post_lol_perks_v1_show_auto_modified_pages_notification**](PluginLolPerksApi.md#post_lol_perks_v1_show_auto_modified_pages_notification) | **Post** /lol-perks/v1/show-auto-modified-pages-notification | 
[**post_lol_perks_v1_update_page_order**](PluginLolPerksApi.md#post_lol_perks_v1_update_page_order) | **Post** /lol-perks/v1/update-page-order | 
[**put_lol_perks_v1_currentpage**](PluginLolPerksApi.md#put_lol_perks_v1_currentpage) | **Put** /lol-perks/v1/currentpage | 
[**put_lol_perks_v1_pages_by_id**](PluginLolPerksApi.md#put_lol_perks_v1_pages_by_id) | **Put** /lol-perks/v1/pages/{id} | 
[**put_lol_perks_v1_pages_validate**](PluginLolPerksApi.md#put_lol_perks_v1_pages_validate) | **Put** /lol-perks/v1/pages/validate | 
[**put_lol_perks_v1_perks_ack_gameplay_updated**](PluginLolPerksApi.md#put_lol_perks_v1_perks_ack_gameplay_updated) | **Put** /lol-perks/v1/perks/ack-gameplay-updated | 
[**put_lol_perks_v1_settings**](PluginLolPerksApi.md#put_lol_perks_v1_settings) | **Put** /lol-perks/v1/settings | 



## delete_lol_perks_v1_pages

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_perks_v1_pages()


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


## delete_lol_perks_v1_pages_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_perks_v1_pages_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_perks_v1_pages_by_id_auto_modified_selections

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_perks_v1_pages_by_id_auto_modified_selections(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_currentpage

> crate::models::LolPerksPerkPageResource get_lol_perks_v1_currentpage()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPerksPerkPageResource**](LolPerksPerkPageResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_customizationlimits

> crate::models::LolPerksCustomizationLimits get_lol_perks_v1_customizationlimits()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPerksCustomizationLimits**](LolPerksCustomizationLimits.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_inventory

> crate::models::LolPerksPlayerInventory get_lol_perks_v1_inventory()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPerksPlayerInventory**](LolPerksPlayerInventory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_pages

> Vec<crate::models::LolPerksPerkPageResource> get_lol_perks_v1_pages()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPerksPerkPageResource>**](LolPerksPerkPageResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_pages_by_id

> crate::models::LolPerksPerkPageResource get_lol_perks_v1_pages_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::LolPerksPerkPageResource**](LolPerksPerkPageResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_perks

> Vec<crate::models::LolPerksPerkUiPerk> get_lol_perks_v1_perks()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPerksPerkUiPerk>**](LolPerksPerkUIPerk.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_perks_disabled

> Vec<i32> get_lol_perks_v1_perks_disabled()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_perks_gameplay_updated

> Vec<i32> get_lol_perks_v1_perks_gameplay_updated()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_schema_version

> i32 get_lol_perks_v1_schema_version()


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


## get_lol_perks_v1_servicesettings

> crate::models::LolPerksServiceSettings get_lol_perks_v1_servicesettings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPerksServiceSettings**](LolPerksServiceSettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_settings

> crate::models::LolPerksUiSettings get_lol_perks_v1_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPerksUiSettings**](LolPerksUISettings.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_perks_v1_show_auto_modified_pages_notification

> bool get_lol_perks_v1_show_auto_modified_pages_notification()


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


## get_lol_perks_v1_styles

> Vec<crate::models::LolPerksPerkUiStyle> get_lol_perks_v1_styles()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPerksPerkUiStyle>**](LolPerksPerkUIStyle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_perks_v1_pages

> crate::models::LolPerksPerkPageResource post_lol_perks_v1_pages(page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | [**LolPerksPerkPageResource**](LolPerksPerkPageResource.md) |  | [required] |

### Return type

[**crate::models::LolPerksPerkPageResource**](LolPerksPerkPageResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_perks_v1_show_auto_modified_pages_notification

> ::std::collections::HashMap<String, serde_json::Value> post_lol_perks_v1_show_auto_modified_pages_notification()


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


## post_lol_perks_v1_update_page_order

> ::std::collections::HashMap<String, serde_json::Value> post_lol_perks_v1_update_page_order(request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**LolPerksUpdatePageOrderRequest**](LolPerksUpdatePageOrderRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_perks_v1_currentpage

> ::std::collections::HashMap<String, serde_json::Value> put_lol_perks_v1_currentpage(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_perks_v1_pages_by_id

> ::std::collections::HashMap<String, serde_json::Value> put_lol_perks_v1_pages_by_id(id, page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**page** | [**LolPerksPerkPageResource**](LolPerksPerkPageResource.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_perks_v1_pages_validate

> crate::models::LolPerksValidatePageNameResponse put_lol_perks_v1_pages_validate(page_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_data** | [**LolPerksValidatePageNameData**](LolPerksValidatePageNameData.md) |  | [required] |

### Return type

[**crate::models::LolPerksValidatePageNameResponse**](LolPerksValidatePageNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_perks_v1_perks_ack_gameplay_updated

> ::std::collections::HashMap<String, serde_json::Value> put_lol_perks_v1_perks_ack_gameplay_updated(ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<i32>**](i32.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_perks_v1_settings

> ::std::collections::HashMap<String, serde_json::Value> put_lol_perks_v1_settings(show_long_descriptions)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**show_long_descriptions** | [**LolPerksUiSettings**](LolPerksUiSettings.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

