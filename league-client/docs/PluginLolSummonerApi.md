# \PluginLolSummonerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_summoner_v1_check_name_availability_by_name**](PluginLolSummonerApi.md#get_lol_summoner_v1_check_name_availability_by_name) | **Get** /lol-summoner/v1/check-name-availability/{name} | 
[**get_lol_summoner_v1_check_name_availability_new_summoners_by_name**](PluginLolSummonerApi.md#get_lol_summoner_v1_check_name_availability_new_summoners_by_name) | **Get** /lol-summoner/v1/check-name-availability-new-summoners/{name} | 
[**get_lol_summoner_v1_current_summoner**](PluginLolSummonerApi.md#get_lol_summoner_v1_current_summoner) | **Get** /lol-summoner/v1/current-summoner | 
[**get_lol_summoner_v1_current_summoner_autofill**](PluginLolSummonerApi.md#get_lol_summoner_v1_current_summoner_autofill) | **Get** /lol-summoner/v1/current-summoner/autofill | 
[**get_lol_summoner_v1_current_summoner_jwt**](PluginLolSummonerApi.md#get_lol_summoner_v1_current_summoner_jwt) | **Get** /lol-summoner/v1/current-summoner/jwt | 
[**get_lol_summoner_v1_current_summoner_reroll_points**](PluginLolSummonerApi.md#get_lol_summoner_v1_current_summoner_reroll_points) | **Get** /lol-summoner/v1/current-summoner/rerollPoints | 
[**get_lol_summoner_v1_current_summoner_summoner_profile**](PluginLolSummonerApi.md#get_lol_summoner_v1_current_summoner_summoner_profile) | **Get** /lol-summoner/v1/current-summoner/summoner-profile | 
[**get_lol_summoner_v1_status**](PluginLolSummonerApi.md#get_lol_summoner_v1_status) | **Get** /lol-summoner/v1/status | 
[**get_lol_summoner_v1_summoner_profile**](PluginLolSummonerApi.md#get_lol_summoner_v1_summoner_profile) | **Get** /lol-summoner/v1/summoner-profile | 
[**get_lol_summoner_v1_summoner_requests_ready**](PluginLolSummonerApi.md#get_lol_summoner_v1_summoner_requests_ready) | **Get** /lol-summoner/v1/summoner-requests-ready | 
[**get_lol_summoner_v1_summoners**](PluginLolSummonerApi.md#get_lol_summoner_v1_summoners) | **Get** /lol-summoner/v1/summoners | 
[**get_lol_summoner_v1_summoners_by_id**](PluginLolSummonerApi.md#get_lol_summoner_v1_summoners_by_id) | **Get** /lol-summoner/v1/summoners/{id} | 
[**get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid**](PluginLolSummonerApi.md#get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid) | **Get** /lol-summoner/v1/summoners-by-puuid-cached/{puuid} | 
[**get_lol_summoner_v2_summoner_icons**](PluginLolSummonerApi.md#get_lol_summoner_v2_summoner_icons) | **Get** /lol-summoner/v2/summoner-icons | 
[**get_lol_summoner_v2_summoner_names**](PluginLolSummonerApi.md#get_lol_summoner_v2_summoner_names) | **Get** /lol-summoner/v2/summoner-names | 
[**get_lol_summoner_v2_summoners**](PluginLolSummonerApi.md#get_lol_summoner_v2_summoners) | **Get** /lol-summoner/v2/summoners | 
[**get_lol_summoner_v2_summoners_puuid_by_puuid**](PluginLolSummonerApi.md#get_lol_summoner_v2_summoners_puuid_by_puuid) | **Get** /lol-summoner/v2/summoners/puuid/{puuid} | 
[**post_lol_summoner_v1_current_summoner_name**](PluginLolSummonerApi.md#post_lol_summoner_v1_current_summoner_name) | **Post** /lol-summoner/v1/current-summoner/name | 
[**post_lol_summoner_v1_current_summoner_summoner_profile**](PluginLolSummonerApi.md#post_lol_summoner_v1_current_summoner_summoner_profile) | **Post** /lol-summoner/v1/current-summoner/summoner-profile | 
[**post_lol_summoner_v1_summoners**](PluginLolSummonerApi.md#post_lol_summoner_v1_summoners) | **Post** /lol-summoner/v1/summoners | 
[**post_lol_summoner_v2_summoners_names**](PluginLolSummonerApi.md#post_lol_summoner_v2_summoners_names) | **Post** /lol-summoner/v2/summoners/names | 
[**post_lol_summoner_v2_summoners_puuid**](PluginLolSummonerApi.md#post_lol_summoner_v2_summoners_puuid) | **Post** /lol-summoner/v2/summoners/puuid | 
[**put_lol_summoner_v1_current_summoner_icon**](PluginLolSummonerApi.md#put_lol_summoner_v1_current_summoner_icon) | **Put** /lol-summoner/v1/current-summoner/icon | 



## get_lol_summoner_v1_check_name_availability_by_name

> bool get_lol_summoner_v1_check_name_availability_by_name(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_check_name_availability_new_summoners_by_name

> bool get_lol_summoner_v1_check_name_availability_new_summoners_by_name(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_current_summoner

> crate::models::LolSummonerSummoner get_lol_summoner_v1_current_summoner()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_current_summoner_autofill

> Vec<crate::models::LolSummonerAutoFillQueueDto> get_lol_summoner_v1_current_summoner_autofill()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolSummonerAutoFillQueueDto>**](LolSummonerAutoFillQueueDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_current_summoner_jwt

> String get_lol_summoner_v1_current_summoner_jwt()


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


## get_lol_summoner_v1_current_summoner_reroll_points

> crate::models::LolSummonerSummonerRerollPoints get_lol_summoner_v1_current_summoner_reroll_points()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolSummonerSummonerRerollPoints**](LolSummonerSummonerRerollPoints.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_current_summoner_summoner_profile

> ::std::collections::HashMap<String, serde_json::Value> get_lol_summoner_v1_current_summoner_summoner_profile()


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


## get_lol_summoner_v1_status

> crate::models::LolSummonerStatus get_lol_summoner_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolSummonerStatus**](LolSummonerStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_summoner_profile

> ::std::collections::HashMap<String, serde_json::Value> get_lol_summoner_v1_summoner_profile(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_summoner_requests_ready

> bool get_lol_summoner_v1_summoner_requests_ready()


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


## get_lol_summoner_v1_summoners

> crate::models::LolSummonerSummoner get_lol_summoner_v1_summoners(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_summoners_by_id

> crate::models::LolSummonerSummoner get_lol_summoner_v1_summoners_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid

> crate::models::LolSummonerSummoner get_lol_summoner_v1_summoners_by_puuid_cached_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v2_summoner_icons

> Vec<crate::models::LolSummonerSummonerIdAndIcon> get_lol_summoner_v2_summoner_icons(ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<i64>**](i64.md) |  | [required] |

### Return type

[**Vec<crate::models::LolSummonerSummonerIdAndIcon>**](LolSummonerSummonerIdAndIcon.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v2_summoner_names

> Vec<crate::models::LolSummonerSummonerIdAndName> get_lol_summoner_v2_summoner_names(ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<i64>**](i64.md) |  | [required] |

### Return type

[**Vec<crate::models::LolSummonerSummonerIdAndName>**](LolSummonerSummonerIdAndName.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v2_summoners

> Vec<crate::models::LolSummonerSummoner> get_lol_summoner_v2_summoners(ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<i64>**](i64.md)> |  |  |

### Return type

[**Vec<crate::models::LolSummonerSummoner>**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_summoner_v2_summoners_puuid_by_puuid

> crate::models::LolSummonerSummoner get_lol_summoner_v2_summoners_puuid_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_summoner_v1_current_summoner_name

> crate::models::LolSummonerSummoner post_lol_summoner_v1_current_summoner_name(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_summoner_v1_current_summoner_summoner_profile

> ::std::collections::HashMap<String, serde_json::Value> post_lol_summoner_v1_current_summoner_summoner_profile(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LolSummonerSummonerProfileUpdate**](LolSummonerSummonerProfileUpdate.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_summoner_v1_summoners

> crate::models::LolSummonerInternalSummoner post_lol_summoner_v1_summoners(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**LolSummonerSummonerRequestedName**](LolSummonerSummonerRequestedName.md) |  | [required] |

### Return type

[**crate::models::LolSummonerInternalSummoner**](LolSummonerInternalSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_summoner_v2_summoners_names

> Vec<crate::models::LolSummonerSummoner> post_lol_summoner_v2_summoners_names(summoner_names)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_names** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::LolSummonerSummoner>**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_summoner_v2_summoners_puuid

> Vec<crate::models::LolSummonerSummoner> post_lol_summoner_v2_summoners_puuid(puuids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**Vec<crate::models::LolSummonerSummoner>**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_summoner_v1_current_summoner_icon

> crate::models::LolSummonerSummoner put_lol_summoner_v1_current_summoner_icon(body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | [**LolSummonerSummonerIcon**](LolSummonerSummonerIcon.md) |  | [required] |

### Return type

[**crate::models::LolSummonerSummoner**](LolSummonerSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

