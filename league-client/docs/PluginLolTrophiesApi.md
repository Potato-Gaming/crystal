# \PluginLolTrophiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_trophies_v1_current_summoner_trophies_profile**](PluginLolTrophiesApi.md#get_lol_trophies_v1_current_summoner_trophies_profile) | **Get** /lol-trophies/v1/current-summoner/trophies/profile | 
[**get_lol_trophies_v1_players_by_puuid_trophies_profile**](PluginLolTrophiesApi.md#get_lol_trophies_v1_players_by_puuid_trophies_profile) | **Get** /lol-trophies/v1/players/{puuid}/trophies/profile | 



## get_lol_trophies_v1_current_summoner_trophies_profile

> crate::models::LolTrophiesTrophyProfileData get_lol_trophies_v1_current_summoner_trophies_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolTrophiesTrophyProfileData**](LolTrophiesTrophyProfileData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_trophies_v1_players_by_puuid_trophies_profile

> crate::models::LolTrophiesTrophyProfileData get_lol_trophies_v1_players_by_puuid_trophies_profile(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolTrophiesTrophyProfileData**](LolTrophiesTrophyProfileData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

