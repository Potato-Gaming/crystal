# \PluginLolBannersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_banners_v1_current_summoner_flags**](PluginLolBannersApi.md#get_lol_banners_v1_current_summoner_flags) | **Get** /lol-banners/v1/current-summoner/flags | 
[**get_lol_banners_v1_current_summoner_flags_equipped**](PluginLolBannersApi.md#get_lol_banners_v1_current_summoner_flags_equipped) | **Get** /lol-banners/v1/current-summoner/flags/equipped | 
[**get_lol_banners_v1_current_summoner_frames_equipped**](PluginLolBannersApi.md#get_lol_banners_v1_current_summoner_frames_equipped) | **Get** /lol-banners/v1/current-summoner/frames/equipped | 
[**get_lol_banners_v1_players_by_puuid_flags_equipped**](PluginLolBannersApi.md#get_lol_banners_v1_players_by_puuid_flags_equipped) | **Get** /lol-banners/v1/players/{puuid}/flags/equipped | 
[**put_lol_banners_v1_current_summoner_flags_equipped**](PluginLolBannersApi.md#put_lol_banners_v1_current_summoner_flags_equipped) | **Put** /lol-banners/v1/current-summoner/flags/equipped | 



## get_lol_banners_v1_current_summoner_flags

> Vec<crate::models::LolBannersBannerFlag> get_lol_banners_v1_current_summoner_flags()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolBannersBannerFlag>**](LolBannersBannerFlag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_banners_v1_current_summoner_flags_equipped

> crate::models::LolBannersBannerFlag get_lol_banners_v1_current_summoner_flags_equipped()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolBannersBannerFlag**](LolBannersBannerFlag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_banners_v1_current_summoner_frames_equipped

> crate::models::LolBannersBannerFrame get_lol_banners_v1_current_summoner_frames_equipped()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolBannersBannerFrame**](LolBannersBannerFrame.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_banners_v1_players_by_puuid_flags_equipped

> crate::models::LolBannersBannerFlag get_lol_banners_v1_players_by_puuid_flags_equipped(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolBannersBannerFlag**](LolBannersBannerFlag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_banners_v1_current_summoner_flags_equipped

> crate::models::LolBannersBannerFlag put_lol_banners_v1_current_summoner_flags_equipped(flag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flag** | [**LolBannersBannerFlag**](LolBannersBannerFlag.md) |  | [required] |

### Return type

[**crate::models::LolBannersBannerFlag**](LolBannersBannerFlag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

