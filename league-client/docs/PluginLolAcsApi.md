# \PluginLolAcsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_acs_v1_delta**](PluginLolAcsApi.md#get_lol_acs_v1_delta) | **Get** /lol-acs/v1/delta | 
[**get_lol_acs_v1_games_by_game_id**](PluginLolAcsApi.md#get_lol_acs_v1_games_by_game_id) | **Get** /lol-acs/v1/games/{gameId} | 
[**get_lol_acs_v1_gametimelines_by_game_id**](PluginLolAcsApi.md#get_lol_acs_v1_gametimelines_by_game_id) | **Get** /lol-acs/v1/gametimelines/{gameId} | 
[**get_lol_acs_v1_matchlists_by_account_id**](PluginLolAcsApi.md#get_lol_acs_v1_matchlists_by_account_id) | **Get** /lol-acs/v1/matchlists/{accountId} | 
[**get_lol_acs_v1_recently_played_champions_by_account_id**](PluginLolAcsApi.md#get_lol_acs_v1_recently_played_champions_by_account_id) | **Get** /lol-acs/v1/recently-played-champions/{accountId} | 
[**get_lol_acs_v2_matchlists**](PluginLolAcsApi.md#get_lol_acs_v2_matchlists) | **Get** /lol-acs/v2/matchlists | 
[**get_lol_acs_v2_recently_played_champions_by_account_id**](PluginLolAcsApi.md#get_lol_acs_v2_recently_played_champions_by_account_id) | **Get** /lol-acs/v2/recently-played-champions/{accountId} | 
[**get_lol_acs_v2_recently_played_champions_current_summoner**](PluginLolAcsApi.md#get_lol_acs_v2_recently_played_champions_current_summoner) | **Get** /lol-acs/v2/recently-played-champions/current-summoner | 
[**get_lol_acs_v2_request_recently_played_champions_by_account_id**](PluginLolAcsApi.md#get_lol_acs_v2_request_recently_played_champions_by_account_id) | **Get** /lol-acs/v2/request-recently-played-champions/{accountId} | 
[**get_lol_acs_v2_request_recently_played_champions_current_summoner**](PluginLolAcsApi.md#get_lol_acs_v2_request_recently_played_champions_current_summoner) | **Get** /lol-acs/v2/request-recently-played-champions/current-summoner | 
[**post_lol_acs_v1_acs_endpoint_override**](PluginLolAcsApi.md#post_lol_acs_v1_acs_endpoint_override) | **Post** /lol-acs/v1/acs-endpoint-override | 



## get_lol_acs_v1_delta

> ::std::collections::HashMap<String, serde_json::Value> get_lol_acs_v1_delta()


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


## get_lol_acs_v1_games_by_game_id

> ::std::collections::HashMap<String, serde_json::Value> get_lol_acs_v1_games_by_game_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v1_gametimelines_by_game_id

> ::std::collections::HashMap<String, serde_json::Value> get_lol_acs_v1_gametimelines_by_game_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v1_matchlists_by_account_id

> ::std::collections::HashMap<String, serde_json::Value> get_lol_acs_v1_matchlists_by_account_id(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v1_recently_played_champions_by_account_id

> crate::models::LolAcsAcsChampionGamesCollection get_lol_acs_v1_recently_played_champions_by_account_id(account_id, force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**force** | **bool** |  | [required] |

### Return type

[**crate::models::LolAcsAcsChampionGamesCollection**](LolAcsAcsChampionGamesCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v2_matchlists

> ::std::collections::HashMap<String, serde_json::Value> get_lol_acs_v2_matchlists(account_id, beg_index, end_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**beg_index** | **i32** |  | [required] |
**end_index** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v2_recently_played_champions_by_account_id

> crate::models::LolAcsAcsChampionGamesCollection get_lol_acs_v2_recently_played_champions_by_account_id(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolAcsAcsChampionGamesCollection**](LolAcsAcsChampionGamesCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v2_recently_played_champions_current_summoner

> crate::models::LolAcsAcsChampionGamesCollection get_lol_acs_v2_recently_played_champions_current_summoner(force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | **bool** |  | [required] |

### Return type

[**crate::models::LolAcsAcsChampionGamesCollection**](LolAcsAcsChampionGamesCollection.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v2_request_recently_played_champions_by_account_id

> get_lol_acs_v2_request_recently_played_champions_by_account_id(account_id, force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**force** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_acs_v2_request_recently_played_champions_current_summoner

> get_lol_acs_v2_request_recently_played_champions_current_summoner(force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_acs_v1_acs_endpoint_override

> ::std::collections::HashMap<String, serde_json::Value> post_lol_acs_v1_acs_endpoint_override(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**LolAcsAcsEndPoint**](LolAcsAcsEndPoint.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

