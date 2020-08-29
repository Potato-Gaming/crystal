# \PluginLolHovercardApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_hovercard_v1_friend_info_by_puuid**](PluginLolHovercardApi.md#get_lol_hovercard_v1_friend_info_by_puuid) | **Get** /lol-hovercard/v1/friend-info/{puuid} | 
[**get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id**](PluginLolHovercardApi.md#get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id) | **Get** /lol-hovercard/v1/friend-info-by-summoner/{summonerId} | 



## get_lol_hovercard_v1_friend_info_by_puuid

> crate::models::LolHovercardHovercardUserInfo get_lol_hovercard_v1_friend_info_by_puuid(puuid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |

### Return type

[**crate::models::LolHovercardHovercardUserInfo**](LolHovercardHovercardUserInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id

> crate::models::LolHovercardHovercardUserInfo get_lol_hovercard_v1_friend_info_by_summoner_by_summoner_id(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolHovercardHovercardUserInfo**](LolHovercardHovercardUserInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

