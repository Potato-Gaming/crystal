# \PluginLolPurchaseWidgetApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_purchase_widget_v1_configuration**](PluginLolPurchaseWidgetApi.md#get_lol_purchase_widget_v1_configuration) | **Get** /lol-purchase-widget/v1/configuration | 
[**get_lol_purchase_widget_v1_order_notifications**](PluginLolPurchaseWidgetApi.md#get_lol_purchase_widget_v1_order_notifications) | **Get** /lol-purchase-widget/v1/order-notifications | 
[**get_lol_purchase_widget_v1_purchasable_item**](PluginLolPurchaseWidgetApi.md#get_lol_purchase_widget_v1_purchasable_item) | **Get** /lol-purchase-widget/v1/purchasable-item | 
[**post_lol_purchase_widget_v1_purchasable_items_by_inventory_type**](PluginLolPurchaseWidgetApi.md#post_lol_purchase_widget_v1_purchasable_items_by_inventory_type) | **Post** /lol-purchase-widget/v1/purchasable-items/{inventoryType} | 
[**post_lol_purchase_widget_v1_purchase_items**](PluginLolPurchaseWidgetApi.md#post_lol_purchase_widget_v1_purchase_items) | **Post** /lol-purchase-widget/v1/purchaseItems | 
[**post_lol_purchase_widget_v1_validate_items**](PluginLolPurchaseWidgetApi.md#post_lol_purchase_widget_v1_validate_items) | **Post** /lol-purchase-widget/v1/validateItems | 
[**post_lol_purchase_widget_v2_purchase_items**](PluginLolPurchaseWidgetApi.md#post_lol_purchase_widget_v2_purchase_items) | **Post** /lol-purchase-widget/v2/purchaseItems | 



## get_lol_purchase_widget_v1_configuration

> crate::models::LolPurchaseWidgetPurchaseWidgetConfig get_lol_purchase_widget_v1_configuration()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPurchaseWidgetPurchaseWidgetConfig**](LolPurchaseWidgetPurchaseWidgetConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_purchase_widget_v1_order_notifications

> Vec<crate::models::LolPurchaseWidgetOrderNotificationResource> get_lol_purchase_widget_v1_order_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPurchaseWidgetOrderNotificationResource>**](LolPurchaseWidgetOrderNotificationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_purchase_widget_v1_purchasable_item

> crate::models::LolPurchaseWidgetPurchasableItem get_lol_purchase_widget_v1_purchasable_item(inventory_type, item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |
**item_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolPurchaseWidgetPurchasableItem**](LolPurchaseWidgetPurchasableItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_purchase_widget_v1_purchasable_items_by_inventory_type

> crate::models::LolPurchaseWidgetItemChoices post_lol_purchase_widget_v1_purchasable_items_by_inventory_type(inventory_type, item_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |
**item_ids** | [**Vec<i64>**](i64.md) |  | [required] |

### Return type

[**crate::models::LolPurchaseWidgetItemChoices**](LolPurchaseWidgetItemChoices.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_purchase_widget_v1_purchase_items

> ::std::collections::HashMap<String, serde_json::Value> post_lol_purchase_widget_v1_purchase_items(purchase_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_request** | [**LolPurchaseWidgetPurchaseRequest**](LolPurchaseWidgetPurchaseRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_purchase_widget_v1_validate_items

> ::std::collections::HashMap<String, serde_json::Value> post_lol_purchase_widget_v1_validate_items(validation_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validation_request** | [**LolPurchaseWidgetValidationRequest**](LolPurchaseWidgetValidationRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_purchase_widget_v2_purchase_items

> ::std::collections::HashMap<String, serde_json::Value> post_lol_purchase_widget_v2_purchase_items(purchase_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**purchase_request** | [**LolPurchaseWidgetPurchaseRequest**](LolPurchaseWidgetPurchaseRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

