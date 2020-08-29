# \PluginLolInventoryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_inventory_v1_champ_select_inventory**](PluginLolInventoryApi.md#get_lol_inventory_v1_champ_select_inventory) | **Get** /lol-inventory/v1/champSelectInventory | 
[**get_lol_inventory_v1_initial_configuration_complete**](PluginLolInventoryApi.md#get_lol_inventory_v1_initial_configuration_complete) | **Get** /lol-inventory/v1/initial-configuration-complete | 
[**get_lol_inventory_v1_inventory**](PluginLolInventoryApi.md#get_lol_inventory_v1_inventory) | **Get** /lol-inventory/v1/inventory | 
[**get_lol_inventory_v1_inventory_emotes**](PluginLolInventoryApi.md#get_lol_inventory_v1_inventory_emotes) | **Get** /lol-inventory/v1/inventory/emotes | 
[**get_lol_inventory_v1_notifications_by_inventory_type**](PluginLolInventoryApi.md#get_lol_inventory_v1_notifications_by_inventory_type) | **Get** /lol-inventory/v1/notifications/{inventoryType} | 
[**get_lol_inventory_v1_players_by_puuid_inventory**](PluginLolInventoryApi.md#get_lol_inventory_v1_players_by_puuid_inventory) | **Get** /lol-inventory/v1/players/{puuid}/inventory | 
[**get_lol_inventory_v1_signed_inventory**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_inventory) | **Get** /lol-inventory/v1/signedInventory | 
[**get_lol_inventory_v1_signed_inventory_cache**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_inventory_cache) | **Get** /lol-inventory/v1/signedInventoryCache | 
[**get_lol_inventory_v1_signed_inventory_simple**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_inventory_simple) | **Get** /lol-inventory/v1/signedInventory/simple | 
[**get_lol_inventory_v1_signed_inventory_tournamentlogos**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_inventory_tournamentlogos) | **Get** /lol-inventory/v1/signedInventory/tournamentlogos | 
[**get_lol_inventory_v1_signed_wallet**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_wallet) | **Get** /lol-inventory/v1/signedWallet | 
[**get_lol_inventory_v1_signed_wallet_by_currency_type**](PluginLolInventoryApi.md#get_lol_inventory_v1_signed_wallet_by_currency_type) | **Get** /lol-inventory/v1/signedWallet/{currencyType} | 
[**get_lol_inventory_v1_wallet**](PluginLolInventoryApi.md#get_lol_inventory_v1_wallet) | **Get** /lol-inventory/v1/wallet | 
[**get_lol_inventory_v1_wallet_by_currency_type**](PluginLolInventoryApi.md#get_lol_inventory_v1_wallet_by_currency_type) | **Get** /lol-inventory/v1/wallet/{currencyType} | 
[**get_lol_inventory_v2_inventory_by_inventory_type**](PluginLolInventoryApi.md#get_lol_inventory_v2_inventory_by_inventory_type) | **Get** /lol-inventory/v2/inventory/{inventoryType} | 
[**post_lol_inventory_v1_notification_acknowledge**](PluginLolInventoryApi.md#post_lol_inventory_v1_notification_acknowledge) | **Post** /lol-inventory/v1/notification/acknowledge | 



## get_lol_inventory_v1_champ_select_inventory

> String get_lol_inventory_v1_champ_select_inventory()


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


## get_lol_inventory_v1_initial_configuration_complete

> bool get_lol_inventory_v1_initial_configuration_complete()


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


## get_lol_inventory_v1_inventory

> Vec<crate::models::LolInventoryInventoryItemWithPayload> get_lol_inventory_v1_inventory(inventory_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_types** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::LolInventoryInventoryItemWithPayload>**](LolInventoryInventoryItemWithPayload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_inventory_emotes

> Vec<crate::models::LolInventoryInventoryItemWithPayload> get_lol_inventory_v1_inventory_emotes()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolInventoryInventoryItemWithPayload>**](LolInventoryInventoryItemWithPayload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_notifications_by_inventory_type

> Vec<crate::models::LolInventoryInventoryNotification> get_lol_inventory_v1_notifications_by_inventory_type(inventory_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolInventoryInventoryNotification>**](LolInventoryInventoryNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_players_by_puuid_inventory

> Vec<crate::models::LolInventoryInventoryItemWithPayload> get_lol_inventory_v1_players_by_puuid_inventory(puuid, inventory_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**inventory_types** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::LolInventoryInventoryItemWithPayload>**](LolInventoryInventoryItemWithPayload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_inventory

> ::std::collections::HashMap<String, String> get_lol_inventory_v1_signed_inventory(inventory_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_types** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_inventory_cache

> ::std::collections::HashMap<String, crate::models::LolInventoryInventoryCacheEntry> get_lol_inventory_v1_signed_inventory_cache()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::LolInventoryInventoryCacheEntry>**](LolInventoryInventoryCacheEntry.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_inventory_simple

> String get_lol_inventory_v1_signed_inventory_simple(inventory_types, timeout_in_seconds, query_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_types** | [**Vec<String>**](String.md) |  | [required] |
**timeout_in_seconds** | Option<**i32**> |  |  |
**query_params** | Option<[**serde_json::Value**](.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_inventory_tournamentlogos

> ::std::collections::HashMap<String, String> get_lol_inventory_v1_signed_inventory_tournamentlogos()


### Parameters

This endpoint does not need any parameter.

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_wallet

> ::std::collections::HashMap<String, String> get_lol_inventory_v1_signed_wallet(currency_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_types** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_signed_wallet_by_currency_type

> ::std::collections::HashMap<String, String> get_lol_inventory_v1_signed_wallet_by_currency_type(currency_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_type** | **String** |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_wallet

> ::std::collections::HashMap<String, i32> get_lol_inventory_v1_wallet(currency_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_types** | [**Vec<String>**](String.md) |  | [required] |

### Return type

**::std::collections::HashMap<String, i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v1_wallet_by_currency_type

> ::std::collections::HashMap<String, i32> get_lol_inventory_v1_wallet_by_currency_type(currency_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency_type** | **String** |  | [required] |

### Return type

**::std::collections::HashMap<String, i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_inventory_v2_inventory_by_inventory_type

> Vec<crate::models::LolInventoryInventoryItemWithPayload> get_lol_inventory_v2_inventory_by_inventory_type(inventory_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inventory_type** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolInventoryInventoryItemWithPayload>**](LolInventoryInventoryItemWithPayload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_inventory_v1_notification_acknowledge

> post_lol_inventory_v1_notification_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

