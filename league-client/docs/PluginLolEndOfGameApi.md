# \PluginLolEndOfGameApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_end_of_game_v1_champion_mastery_updates**](PluginLolEndOfGameApi.md#get_lol_end_of_game_v1_champion_mastery_updates) | **Get** /lol-end-of-game/v1/champion-mastery-updates | 
[**get_lol_end_of_game_v1_eog_stats_block**](PluginLolEndOfGameApi.md#get_lol_end_of_game_v1_eog_stats_block) | **Get** /lol-end-of-game/v1/eog-stats-block | 
[**get_lol_end_of_game_v1_gameclient_eog_stats_block**](PluginLolEndOfGameApi.md#get_lol_end_of_game_v1_gameclient_eog_stats_block) | **Get** /lol-end-of-game/v1/gameclient-eog-stats-block | 
[**get_lol_end_of_game_v1_reported_players**](PluginLolEndOfGameApi.md#get_lol_end_of_game_v1_reported_players) | **Get** /lol-end-of-game/v1/reported-players | 
[**get_lol_end_of_game_v1_tft_eog_stats**](PluginLolEndOfGameApi.md#get_lol_end_of_game_v1_tft_eog_stats) | **Get** /lol-end-of-game/v1/tft-eog-stats | 
[**post_lol_end_of_game_v1_gameclient_eog_stats_block**](PluginLolEndOfGameApi.md#post_lol_end_of_game_v1_gameclient_eog_stats_block) | **Post** /lol-end-of-game/v1/gameclient-eog-stats-block | 
[**post_lol_end_of_game_v1_state_dismiss_stats**](PluginLolEndOfGameApi.md#post_lol_end_of_game_v1_state_dismiss_stats) | **Post** /lol-end-of-game/v1/state/dismiss-stats | 
[**post_lol_end_of_game_v2_player_complaints**](PluginLolEndOfGameApi.md#post_lol_end_of_game_v2_player_complaints) | **Post** /lol-end-of-game/v2/player-complaints | 



## get_lol_end_of_game_v1_champion_mastery_updates

> crate::models::LolEndOfGameChampionMasteryUpdate get_lol_end_of_game_v1_champion_mastery_updates()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEndOfGameChampionMasteryUpdate**](LolEndOfGameChampionMasteryUpdate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_end_of_game_v1_eog_stats_block

> crate::models::LolEndOfGameEndOfGameStats get_lol_end_of_game_v1_eog_stats_block()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEndOfGameEndOfGameStats**](LolEndOfGameEndOfGameStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_end_of_game_v1_gameclient_eog_stats_block

> crate::models::LolEndOfGameGameClientEndOfGameStats get_lol_end_of_game_v1_gameclient_eog_stats_block()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEndOfGameGameClientEndOfGameStats**](LolEndOfGameGameClientEndOfGameStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_end_of_game_v1_reported_players

> Vec<i64> get_lol_end_of_game_v1_reported_players()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i64>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_end_of_game_v1_tft_eog_stats

> crate::models::LolEndOfGameTftEndOfGameViewModel get_lol_end_of_game_v1_tft_eog_stats()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEndOfGameTftEndOfGameViewModel**](LolEndOfGameTFTEndOfGameViewModel.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_end_of_game_v1_gameclient_eog_stats_block

> ::std::collections::HashMap<String, serde_json::Value> post_lol_end_of_game_v1_gameclient_eog_stats_block(stats)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stats** | [**LolEndOfGameGameClientEndOfGameStats**](LolEndOfGameGameClientEndOfGameStats.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_end_of_game_v1_state_dismiss_stats

> ::std::collections::HashMap<String, serde_json::Value> post_lol_end_of_game_v1_state_dismiss_stats()


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


## post_lol_end_of_game_v2_player_complaints

> crate::models::LolEndOfGameEndOfGamePlayerComplaintV2 post_lol_end_of_game_v2_player_complaints(complaint)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**complaint** | [**LolEndOfGameEndOfGamePlayerComplaintV2**](LolEndOfGameEndOfGamePlayerComplaintV2.md) |  | [required] |

### Return type

[**crate::models::LolEndOfGameEndOfGamePlayerComplaintV2**](LolEndOfGameEndOfGamePlayerComplaintV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

