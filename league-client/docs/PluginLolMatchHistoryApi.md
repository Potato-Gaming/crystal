# \PluginLolMatchHistoryApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_match_history_v1_delta**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_delta) | **Get** /lol-match-history/v1/delta | 
[**get_lol_match_history_v1_friend_matchlists_by_account_id**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_friend_matchlists_by_account_id) | **Get** /lol-match-history/v1/friend-matchlists/{accountId} | 
[**get_lol_match_history_v1_game_timelines_by_game_id**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_game_timelines_by_game_id) | **Get** /lol-match-history/v1/game-timelines/{gameId} | 
[**get_lol_match_history_v1_games_by_game_id**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_games_by_game_id) | **Get** /lol-match-history/v1/games/{gameId} | 
[**get_lol_match_history_v1_matchlist**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_matchlist) | **Get** /lol-match-history/v1/matchlist | 
[**get_lol_match_history_v1_recently_played_summoners**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_recently_played_summoners) | **Get** /lol-match-history/v1/recently-played-summoners | 
[**get_lol_match_history_v1_web_url**](PluginLolMatchHistoryApi.md#get_lol_match_history_v1_web_url) | **Get** /lol-match-history/v1/web-url | 
[**get_lol_match_history_v2_matchlist**](PluginLolMatchHistoryApi.md#get_lol_match_history_v2_matchlist) | **Get** /lol-match-history/v2/matchlist | 



## get_lol_match_history_v1_delta

> crate::models::LolMatchHistoryMatchHistoryPlayerDelta get_lol_match_history_v1_delta()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolMatchHistoryMatchHistoryPlayerDelta**](LolMatchHistoryMatchHistoryPlayerDelta.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_friend_matchlists_by_account_id

> crate::models::LolMatchHistoryMatchHistoryList get_lol_match_history_v1_friend_matchlists_by_account_id(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolMatchHistoryMatchHistoryList**](LolMatchHistoryMatchHistoryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_game_timelines_by_game_id

> crate::models::LolMatchHistoryMatchHistoryTimelineFrames get_lol_match_history_v1_game_timelines_by_game_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolMatchHistoryMatchHistoryTimelineFrames**](LolMatchHistoryMatchHistoryTimelineFrames.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_games_by_game_id

> crate::models::LolMatchHistoryMatchHistoryGame get_lol_match_history_v1_games_by_game_id(game_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**game_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolMatchHistoryMatchHistoryGame**](LolMatchHistoryMatchHistoryGame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_matchlist

> crate::models::LolMatchHistoryMatchHistoryList get_lol_match_history_v1_matchlist()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolMatchHistoryMatchHistoryList**](LolMatchHistoryMatchHistoryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_recently_played_summoners

> Vec<crate::models::LolMatchHistoryRecentlyPlayedSummoner> get_lol_match_history_v1_recently_played_summoners()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolMatchHistoryRecentlyPlayedSummoner>**](LolMatchHistoryRecentlyPlayedSummoner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_match_history_v1_web_url

> String get_lol_match_history_v1_web_url()


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


## get_lol_match_history_v2_matchlist

> crate::models::LolMatchHistoryMatchHistoryList get_lol_match_history_v2_matchlist(beg_index, end_index)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**beg_index** | **i32** |  | [required] |
**end_index** | **i32** |  | [required] |

### Return type

[**crate::models::LolMatchHistoryMatchHistoryList**](LolMatchHistoryMatchHistoryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

