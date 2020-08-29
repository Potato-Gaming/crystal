# \PluginLolClubsPublicApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_clubs_public_v1_clubs_public**](PluginLolClubsPublicApi.md#get_lol_clubs_public_v1_clubs_public) | **Get** /lol-clubs-public/v1/clubs/public | 
[**get_lol_clubs_public_v1_clubs_public_by_summoner_id**](PluginLolClubsPublicApi.md#get_lol_clubs_public_v1_clubs_public_by_summoner_id) | **Get** /lol-clubs-public/v1/clubs/public/{summonerId} | 
[**get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag**](PluginLolClubsPublicApi.md#get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag) | **Get** /lol-clubs-public/v1/clubs/public/{summonerId}/tag | 



## get_lol_clubs_public_v1_clubs_public

> Vec<crate::models::LolClubsPublicClubsPublicData> get_lol_clubs_public_v1_clubs_public(summoner_names)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_names** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolClubsPublicClubsPublicData>**](LolClubsPublicClubsPublicData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_public_v1_clubs_public_by_summoner_id

> crate::models::LolClubsPublicClubsPublicData get_lol_clubs_public_v1_clubs_public_by_summoner_id(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsPublicClubsPublicData**](LolClubsPublicClubsPublicData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag

> crate::models::LolClubsPublicClubTag get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsPublicClubTag**](LolClubsPublicClubTag.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

