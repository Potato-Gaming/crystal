# \PluginLolCatalogApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_catalog_v1_item_details**](PluginLolCatalogApi.md#get_lol_catalog_v1_item_details) | **Get** /lol-catalog/v1/item-details | 
[**get_lol_catalog_v1_items**](PluginLolCatalogApi.md#get_lol_catalog_v1_items) | **Get** /lol-catalog/v1/items | 
[**get_lol_catalog_v1_items_by_inventory_type**](PluginLolCatalogApi.md#get_lol_catalog_v1_items_by_inventory_type) | **Get** /lol-catalog/v1/items/{inventoryType} | 



## get_lol_catalog_v1_item_details

> crate::models::LolCatalogCatalogPluginItemWithDetails get_lol_catalog_v1_item_details(inventory_type, item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |
**item_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCatalogCatalogPluginItemWithDetails**](LolCatalogCatalogPluginItemWithDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_catalog_v1_items

> Vec<crate::models::LolCatalogItemChoiceDetails> get_lol_catalog_v1_items(inventory_type, item_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |
**item_ids** | [**Vec<i64>**](i64.md) |  | [required] |

### Return type

[**Vec<crate::models::LolCatalogItemChoiceDetails>**](LolCatalogItemChoiceDetails.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_catalog_v1_items_by_inventory_type

> Vec<crate::models::LolCatalogCatalogPluginItem> get_lol_catalog_v1_items_by_inventory_type(inventory_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolCatalogCatalogPluginItem>**](LolCatalogCatalogPluginItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

