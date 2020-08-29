# \PluginLolStoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_store_v1_by_page_type**](PluginLolStoreApi.md#get_lol_store_v1_by_page_type) | **Get** /lol-store/v1/{pageType} | 
[**get_lol_store_v1_catalog**](PluginLolStoreApi.md#get_lol_store_v1_catalog) | **Get** /lol-store/v1/catalog | 
[**get_lol_store_v1_catalog_by_instance_ids**](PluginLolStoreApi.md#get_lol_store_v1_catalog_by_instance_ids) | **Get** /lol-store/v1/catalogByInstanceIds | 
[**get_lol_store_v1_catalog_by_inventory_type**](PluginLolStoreApi.md#get_lol_store_v1_catalog_by_inventory_type) | **Get** /lol-store/v1/catalog/{inventoryType} | 
[**get_lol_store_v1_catalog_sales**](PluginLolStoreApi.md#get_lol_store_v1_catalog_sales) | **Get** /lol-store/v1/catalog/sales | 
[**get_lol_store_v1_get_store_url**](PluginLolStoreApi.md#get_lol_store_v1_get_store_url) | **Get** /lol-store/v1/getStoreUrl | 
[**get_lol_store_v1_giftablefriends**](PluginLolStoreApi.md#get_lol_store_v1_giftablefriends) | **Get** /lol-store/v1/giftablefriends | 
[**get_lol_store_v1_item_keys_from_instance_ids**](PluginLolStoreApi.md#get_lol_store_v1_item_keys_from_instance_ids) | **Get** /lol-store/v1/itemKeysFromInstanceIds | 
[**get_lol_store_v1_last_page**](PluginLolStoreApi.md#get_lol_store_v1_last_page) | **Get** /lol-store/v1/lastPage | 
[**get_lol_store_v1_offers**](PluginLolStoreApi.md#get_lol_store_v1_offers) | **Get** /lol-store/v1/offers | 
[**get_lol_store_v1_order_notifications**](PluginLolStoreApi.md#get_lol_store_v1_order_notifications) | **Get** /lol-store/v1/order-notifications | 
[**get_lol_store_v1_order_notifications_by_id**](PluginLolStoreApi.md#get_lol_store_v1_order_notifications_by_id) | **Get** /lol-store/v1/order-notifications/{id} | 
[**get_lol_store_v1_payment_details**](PluginLolStoreApi.md#get_lol_store_v1_payment_details) | **Get** /lol-store/v1/paymentDetails | 
[**get_lol_store_v1_skins_by_skin_id**](PluginLolStoreApi.md#get_lol_store_v1_skins_by_skin_id) | **Get** /lol-store/v1/skins/{skinId} | 
[**get_lol_store_v1_status**](PluginLolStoreApi.md#get_lol_store_v1_status) | **Get** /lol-store/v1/status | 
[**get_lol_store_v1_store_ready**](PluginLolStoreApi.md#get_lol_store_v1_store_ready) | **Get** /lol-store/v1/store-ready | 
[**get_lol_store_v1_wallet**](PluginLolStoreApi.md#get_lol_store_v1_wallet) | **Get** /lol-store/v1/wallet | 
[**post_lol_store_v1_last_page**](PluginLolStoreApi.md#post_lol_store_v1_last_page) | **Post** /lol-store/v1/lastPage | 
[**post_lol_store_v1_notifications_acknowledge**](PluginLolStoreApi.md#post_lol_store_v1_notifications_acknowledge) | **Post** /lol-store/v1/notifications/acknowledge | 
[**post_lol_store_v1_skins_by_skin_id_purchase**](PluginLolStoreApi.md#post_lol_store_v1_skins_by_skin_id_purchase) | **Post** /lol-store/v1/skins/{skinId}/purchase | 



## get_lol_store_v1_by_page_type

> ::std::collections::HashMap<String, serde_json::Value> get_lol_store_v1_by_page_type(page_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_type** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_catalog

> Vec<crate::models::LolStoreCatalogItem> get_lol_store_v1_catalog(inventory_type, item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | Option<[**Vec<String>**](String.md)> |  |  |
**item_id** | Option<[**Vec<i32>**](i32.md)> |  |  |

### Return type

[**Vec<crate::models::LolStoreCatalogItem>**](LolStoreCatalogItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_catalog_by_instance_ids

> Vec<crate::models::LolStoreCatalogItem> get_lol_store_v1_catalog_by_instance_ids(instance_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::LolStoreCatalogItem>**](LolStoreCatalogItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_catalog_by_inventory_type

> Vec<crate::models::LolStoreCatalogItem> get_lol_store_v1_catalog_by_inventory_type(inventory_type, item_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |
**item_ids** | [**Vec<i32>**](i32.md) |  | [required] |

### Return type

[**Vec<crate::models::LolStoreCatalogItem>**](LolStoreCatalogItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_catalog_sales

> Vec<crate::models::LolStoreItemSale> get_lol_store_v1_catalog_sales()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolStoreItemSale>**](LolStoreItemSale.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_get_store_url

> String get_lol_store_v1_get_store_url()


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


## get_lol_store_v1_giftablefriends

> Vec<crate::models::LolStoreGiftingFriend> get_lol_store_v1_giftablefriends()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolStoreGiftingFriend>**](LolStoreGiftingFriend.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_item_keys_from_instance_ids

> ::std::collections::HashMap<String, crate::models::LolStoreItemKey> get_lol_store_v1_item_keys_from_instance_ids(instance_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::LolStoreItemKey>**](LolStoreItemKey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_last_page

> String get_lol_store_v1_last_page()


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


## get_lol_store_v1_offers

> Vec<crate::models::LolStoreCapOffer> get_lol_store_v1_offers(inventory_type_uui_ds)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type_uui_ds** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<crate::models::LolStoreCapOffer>**](LolStoreCapOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_order_notifications

> Vec<crate::models::LolStoreOrderNotificationResource> get_lol_store_v1_order_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolStoreOrderNotificationResource>**](LolStoreOrderNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_order_notifications_by_id

> crate::models::LolStoreOrderNotificationResource get_lol_store_v1_order_notifications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolStoreOrderNotificationResource**](LolStoreOrderNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_payment_details

> ::std::collections::HashMap<String, serde_json::Value> get_lol_store_v1_payment_details(action, gift_recipient_account_id, gift_message)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**action** | **String** |  | [required] |
**gift_recipient_account_id** | Option<**i64**> |  |  |
**gift_message** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_skins_by_skin_id

> crate::models::LolStoreCatalogItem get_lol_store_v1_skins_by_skin_id(skin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skin_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolStoreCatalogItem**](LolStoreCatalogItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_status

> crate::models::LolStoreStoreStatus get_lol_store_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolStoreStoreStatus**](LolStoreStoreStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_store_v1_store_ready

> bool get_lol_store_v1_store_ready()


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


## get_lol_store_v1_wallet

> crate::models::LolStoreWallet get_lol_store_v1_wallet()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolStoreWallet**](LolStoreWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_store_v1_last_page

> post_lol_store_v1_last_page(page_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_type** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_store_v1_notifications_acknowledge

> ::std::collections::HashMap<String, serde_json::Value> post_lol_store_v1_notifications_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_store_v1_skins_by_skin_id_purchase

> ::std::collections::HashMap<String, serde_json::Value> post_lol_store_v1_skins_by_skin_id_purchase(skin_id, cost)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skin_id** | **i32** |  | [required] |
**cost** | [**LolStoreItemCost**](LolStoreItemCost.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

