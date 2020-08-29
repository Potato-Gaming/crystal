# \PluginLolSpectatorApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_spectator_v1_spectate**](PluginLolSpectatorApi.md#get_lol_spectator_v1_spectate) | **Get** /lol-spectator/v1/spectate | 
[**post_lol_spectator_v1_buddy_spectate**](PluginLolSpectatorApi.md#post_lol_spectator_v1_buddy_spectate) | **Post** /lol-spectator/v1/buddy/spectate | 
[**post_lol_spectator_v1_spectate_launch**](PluginLolSpectatorApi.md#post_lol_spectator_v1_spectate_launch) | **Post** /lol-spectator/v1/spectate/launch | 



## get_lol_spectator_v1_spectate

> crate::models::LolSpectatorSpectateGameInfo get_lol_spectator_v1_spectate()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolSpectatorSpectateGameInfo**](LolSpectatorSpectateGameInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_spectator_v1_buddy_spectate

> crate::models::LolSpectatorSummonerOrTeamAvailabilty post_lol_spectator_v1_buddy_spectate(summoner_or_team_names)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_or_team_names** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**crate::models::LolSpectatorSummonerOrTeamAvailabilty**](LolSpectatorSummonerOrTeamAvailabilty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_spectator_v1_spectate_launch

> ::std::collections::HashMap<String, serde_json::Value> post_lol_spectator_v1_spectate_launch(spectate_game_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**spectate_game_info** | [**LolSpectatorSpectateGameInfo**](LolSpectatorSpectateGameInfo.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

