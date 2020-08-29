# \PluginLolItemSetsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_item_sets_v1_item_sets_by_summoner_id_sets**](PluginLolItemSetsApi.md#get_lol_item_sets_v1_item_sets_by_summoner_id_sets) | **Get** /lol-item-sets/v1/item-sets/{summonerId}/sets | 
[**post_lol_item_sets_v1_item_sets_by_summoner_id_sets**](PluginLolItemSetsApi.md#post_lol_item_sets_v1_item_sets_by_summoner_id_sets) | **Post** /lol-item-sets/v1/item-sets/{summonerId}/sets | 
[**post_lol_item_sets_v1_item_sets_by_summoner_id_validate**](PluginLolItemSetsApi.md#post_lol_item_sets_v1_item_sets_by_summoner_id_validate) | **Post** /lol-item-sets/v1/item-sets/{summonerId}/validate | 
[**put_lol_item_sets_v1_item_sets_by_summoner_id_sets**](PluginLolItemSetsApi.md#put_lol_item_sets_v1_item_sets_by_summoner_id_sets) | **Put** /lol-item-sets/v1/item-sets/{summonerId}/sets | 



## get_lol_item_sets_v1_item_sets_by_summoner_id_sets

> crate::models::LolItemSetsItemSets get_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolItemSetsItemSets**](LolItemSetsItemSets.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_item_sets_v1_item_sets_by_summoner_id_sets

> post_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id, item_set)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**item_set** | [**LolItemSetsItemSet**](LolItemSetsItemSet.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_item_sets_v1_item_sets_by_summoner_id_validate

> crate::models::LolItemSetsValidateItemSetNameResponse post_lol_item_sets_v1_item_sets_by_summoner_id_validate(summoner_id, input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**input** | [**LolItemSetsValidateItemSetNameInput**](LolItemSetsValidateItemSetNameInput.md) |  | [required] |

### Return type

[**crate::models::LolItemSetsValidateItemSetNameResponse**](LolItemSetsValidateItemSetNameResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_item_sets_v1_item_sets_by_summoner_id_sets

> put_lol_item_sets_v1_item_sets_by_summoner_id_sets(summoner_id, item_sets)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**item_sets** | [**LolItemSetsItemSets**](LolItemSetsItemSets.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

