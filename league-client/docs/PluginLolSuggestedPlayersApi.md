# \PluginLolSuggestedPlayersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_suggested_players_v1_suggested_players_by_summoner_id**](PluginLolSuggestedPlayersApi.md#delete_lol_suggested_players_v1_suggested_players_by_summoner_id) | **Delete** /lol-suggested-players/v1/suggested-players/{summonerId} | 
[**get_lol_suggested_players_v1_suggested_players**](PluginLolSuggestedPlayersApi.md#get_lol_suggested_players_v1_suggested_players) | **Get** /lol-suggested-players/v1/suggested-players | 
[**post_lol_suggested_players_v1_reported_player**](PluginLolSuggestedPlayersApi.md#post_lol_suggested_players_v1_reported_player) | **Post** /lol-suggested-players/v1/reported-player | 
[**post_lol_suggested_players_v1_victorious_comrade**](PluginLolSuggestedPlayersApi.md#post_lol_suggested_players_v1_victorious_comrade) | **Post** /lol-suggested-players/v1/victorious-comrade | 



## delete_lol_suggested_players_v1_suggested_players_by_summoner_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_suggested_players_v1_suggested_players_by_summoner_id(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_suggested_players_v1_suggested_players

> Vec<crate::models::LolSuggestedPlayersSuggestedPlayersSuggestedPlayer> get_lol_suggested_players_v1_suggested_players()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolSuggestedPlayersSuggestedPlayersSuggestedPlayer>**](LolSuggestedPlayersSuggestedPlayersSuggestedPlayer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_suggested_players_v1_reported_player

> post_lol_suggested_players_v1_reported_player(resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | [**LolSuggestedPlayersSuggestedPlayersReportedPlayer**](LolSuggestedPlayersSuggestedPlayersReportedPlayer.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_suggested_players_v1_victorious_comrade

> post_lol_suggested_players_v1_victorious_comrade(resource)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource** | [**LolSuggestedPlayersSuggestedPlayersVictoriousComrade**](LolSuggestedPlayersSuggestedPlayersVictoriousComrade.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

