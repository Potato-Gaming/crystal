# \PluginLolLoadoutsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_loadouts_v4_loadouts_by_id**](PluginLolLoadoutsApi.md#delete_lol_loadouts_v4_loadouts_by_id) | **Delete** /lol-loadouts/v4/loadouts/{id} | 
[**get_lol_loadouts_v1_loadouts_ready**](PluginLolLoadoutsApi.md#get_lol_loadouts_v1_loadouts_ready) | **Get** /lol-loadouts/v1/loadouts-ready | 
[**get_lol_loadouts_v4_loadouts_by_loadout_id**](PluginLolLoadoutsApi.md#get_lol_loadouts_v4_loadouts_by_loadout_id) | **Get** /lol-loadouts/v4/loadouts/{loadoutId} | 
[**get_lol_loadouts_v4_loadouts_scope_account**](PluginLolLoadoutsApi.md#get_lol_loadouts_v4_loadouts_scope_account) | **Get** /lol-loadouts/v4/loadouts/scope/account | 
[**get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id**](PluginLolLoadoutsApi.md#get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id) | **Get** /lol-loadouts/v4/loadouts/scope/{scope}/{scopeItemId} | 
[**patch_lol_loadouts_v4_loadouts_by_id**](PluginLolLoadoutsApi.md#patch_lol_loadouts_v4_loadouts_by_id) | **Patch** /lol-loadouts/v4/loadouts/{id} | 
[**post_lol_loadouts_v4_loadouts**](PluginLolLoadoutsApi.md#post_lol_loadouts_v4_loadouts) | **Post** /lol-loadouts/v4/loadouts | 
[**put_lol_loadouts_v4_loadouts_by_id**](PluginLolLoadoutsApi.md#put_lol_loadouts_v4_loadouts_by_id) | **Put** /lol-loadouts/v4/loadouts/{id} | 



## delete_lol_loadouts_v4_loadouts_by_id

> delete_lol_loadouts_v4_loadouts_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loadouts_v1_loadouts_ready

> bool get_lol_loadouts_v1_loadouts_ready()


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


## get_lol_loadouts_v4_loadouts_by_loadout_id

> crate::models::LolLoadoutsScopedLoadout get_lol_loadouts_v4_loadouts_by_loadout_id(loadout_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loadout_id** | **String** |  | [required] |

### Return type

[**crate::models::LolLoadoutsScopedLoadout**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loadouts_v4_loadouts_scope_account

> Vec<crate::models::LolLoadoutsScopedLoadout> get_lol_loadouts_v4_loadouts_scope_account()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLoadoutsScopedLoadout>**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id

> Vec<crate::models::LolLoadoutsScopedLoadout> get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id(scope, scope_item_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | **String** |  | [required] |
**scope_item_id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::LolLoadoutsScopedLoadout>**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_loadouts_v4_loadouts_by_id

> crate::models::LolLoadoutsScopedLoadout patch_lol_loadouts_v4_loadouts_by_id(id, loadout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**loadout** | [**LolLoadoutsUpdateLoadoutDto**](LolLoadoutsUpdateLoadoutDto.md) |  | [required] |

### Return type

[**crate::models::LolLoadoutsScopedLoadout**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loadouts_v4_loadouts

> crate::models::LolLoadoutsScopedLoadout post_lol_loadouts_v4_loadouts(loadout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loadout** | [**LolLoadoutsCreateLoadoutDto**](LolLoadoutsCreateLoadoutDto.md) |  | [required] |

### Return type

[**crate::models::LolLoadoutsScopedLoadout**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_loadouts_v4_loadouts_by_id

> crate::models::LolLoadoutsScopedLoadout put_lol_loadouts_v4_loadouts_by_id(id, loadout)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**loadout** | [**LolLoadoutsUpdateLoadoutDto**](LolLoadoutsUpdateLoadoutDto.md) |  | [required] |

### Return type

[**crate::models::LolLoadoutsScopedLoadout**](LolLoadoutsScopedLoadout.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

