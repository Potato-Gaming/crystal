# \PluginLolCosmeticsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_cosmetics_v1_selection_companion**](PluginLolCosmeticsApi.md#delete_lol_cosmetics_v1_selection_companion) | **Delete** /lol-cosmetics/v1/selection/companion | 
[**delete_lol_cosmetics_v1_selection_tft_damage_skin**](PluginLolCosmeticsApi.md#delete_lol_cosmetics_v1_selection_tft_damage_skin) | **Delete** /lol-cosmetics/v1/selection/tft-damage-skin | 
[**delete_lol_cosmetics_v1_selection_tft_map_skin**](PluginLolCosmeticsApi.md#delete_lol_cosmetics_v1_selection_tft_map_skin) | **Delete** /lol-cosmetics/v1/selection/tft-map-skin | 
[**get_lol_cosmetics_v1_inventories_by_set_name_companions**](PluginLolCosmeticsApi.md#get_lol_cosmetics_v1_inventories_by_set_name_companions) | **Get** /lol-cosmetics/v1/inventories/{setName}/companions | 
[**get_lol_cosmetics_v1_inventories_by_set_name_damage_skins**](PluginLolCosmeticsApi.md#get_lol_cosmetics_v1_inventories_by_set_name_damage_skins) | **Get** /lol-cosmetics/v1/inventories/{setName}/damage-skins | 
[**get_lol_cosmetics_v1_inventories_by_set_name_map_skins**](PluginLolCosmeticsApi.md#get_lol_cosmetics_v1_inventories_by_set_name_map_skins) | **Get** /lol-cosmetics/v1/inventories/{setName}/map-skins | 
[**put_lol_cosmetics_v1_selection_companion**](PluginLolCosmeticsApi.md#put_lol_cosmetics_v1_selection_companion) | **Put** /lol-cosmetics/v1/selection/companion | 
[**put_lol_cosmetics_v1_selection_tft_damage_skin**](PluginLolCosmeticsApi.md#put_lol_cosmetics_v1_selection_tft_damage_skin) | **Put** /lol-cosmetics/v1/selection/tft-damage-skin | 
[**put_lol_cosmetics_v1_selection_tft_map_skin**](PluginLolCosmeticsApi.md#put_lol_cosmetics_v1_selection_tft_map_skin) | **Put** /lol-cosmetics/v1/selection/tft-map-skin | 



## delete_lol_cosmetics_v1_selection_companion

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_cosmetics_v1_selection_companion()


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


## delete_lol_cosmetics_v1_selection_tft_damage_skin

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_cosmetics_v1_selection_tft_damage_skin()


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


## delete_lol_cosmetics_v1_selection_tft_map_skin

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_cosmetics_v1_selection_tft_map_skin()


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


## get_lol_cosmetics_v1_inventories_by_set_name_companions

> crate::models::LolCosmeticsCompanionsGroupedViewModel get_lol_cosmetics_v1_inventories_by_set_name_companions(set_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_name** | **String** |  | [required] |

### Return type

[**crate::models::LolCosmeticsCompanionsGroupedViewModel**](LolCosmeticsCompanionsGroupedViewModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_cosmetics_v1_inventories_by_set_name_damage_skins

> crate::models::LolCosmeticsTftDamageSkinGroupedViewModel get_lol_cosmetics_v1_inventories_by_set_name_damage_skins(set_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_name** | **String** |  | [required] |

### Return type

[**crate::models::LolCosmeticsTftDamageSkinGroupedViewModel**](LolCosmeticsTFTDamageSkinGroupedViewModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_cosmetics_v1_inventories_by_set_name_map_skins

> crate::models::LolCosmeticsTftMapSkinGroupedViewModel get_lol_cosmetics_v1_inventories_by_set_name_map_skins(set_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_name** | **String** |  | [required] |

### Return type

[**crate::models::LolCosmeticsTftMapSkinGroupedViewModel**](LolCosmeticsTFTMapSkinGroupedViewModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_cosmetics_v1_selection_companion

> ::std::collections::HashMap<String, serde_json::Value> put_lol_cosmetics_v1_selection_companion(item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_cosmetics_v1_selection_tft_damage_skin

> ::std::collections::HashMap<String, serde_json::Value> put_lol_cosmetics_v1_selection_tft_damage_skin(item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_cosmetics_v1_selection_tft_map_skin

> ::std::collections::HashMap<String, serde_json::Value> put_lol_cosmetics_v1_selection_tft_map_skin(item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

