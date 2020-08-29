# \PluginLolMissionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_missions_v1_data**](PluginLolMissionsApi.md#get_lol_missions_v1_data) | **Get** /lol-missions/v1/data | 
[**get_lol_missions_v1_grants**](PluginLolMissionsApi.md#get_lol_missions_v1_grants) | **Get** /lol-missions/v1/grants | 
[**get_lol_missions_v1_missions**](PluginLolMissionsApi.md#get_lol_missions_v1_missions) | **Get** /lol-missions/v1/missions | 
[**get_lol_missions_v1_series**](PluginLolMissionsApi.md#get_lol_missions_v1_series) | **Get** /lol-missions/v1/series | 
[**get_lol_tft_v2_tft_battlepass**](PluginLolMissionsApi.md#get_lol_tft_v2_tft_battlepass) | **Get** /lol-tft/v2/tft/battlepass | 
[**patch_lol_missions_v1_grants_by_grant_id_viewed**](PluginLolMissionsApi.md#patch_lol_missions_v1_grants_by_grant_id_viewed) | **Patch** /lol-missions/v1/grants/{grantId}/viewed | 
[**post_lol_missions_v1_force**](PluginLolMissionsApi.md#post_lol_missions_v1_force) | **Post** /lol-missions/v1/force | 
[**put_lol_missions_v1_player**](PluginLolMissionsApi.md#put_lol_missions_v1_player) | **Put** /lol-missions/v1/player | 
[**put_lol_missions_v1_player_by_mission_id**](PluginLolMissionsApi.md#put_lol_missions_v1_player_by_mission_id) | **Put** /lol-missions/v1/player/{missionId} | 
[**put_lol_missions_v2_player_opt**](PluginLolMissionsApi.md#put_lol_missions_v2_player_opt) | **Put** /lol-missions/v2/player/opt | 



## get_lol_missions_v1_data

> crate::models::PlayerMissionEligibilityData get_lol_missions_v1_data()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PlayerMissionEligibilityData**](PlayerMissionEligibilityData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_missions_v1_grants

> Vec<crate::models::LolMissionsRewardGrant> get_lol_missions_v1_grants()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolMissionsRewardGrant>**](LolMissionsRewardGrant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_missions_v1_missions

> Vec<crate::models::PlayerMissionDto> get_lol_missions_v1_missions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PlayerMissionDto>**](PlayerMissionDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_missions_v1_series

> Vec<crate::models::SeriesDto> get_lol_missions_v1_series()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SeriesDto>**](SeriesDTO.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_tft_v2_tft_battlepass

> crate::models::LolMissionsTftPaidBattlepass get_lol_tft_v2_tft_battlepass()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolMissionsTftPaidBattlepass**](LolMissionsTftPaidBattlepass.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_missions_v1_grants_by_grant_id_viewed

> patch_lol_missions_v1_grants_by_grant_id_viewed(grant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_missions_v1_force

> post_lol_missions_v1_force()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_missions_v1_player

> put_lol_missions_v1_player(ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**IdsDto**](IdsDto.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_missions_v1_player_by_mission_id

> put_lol_missions_v1_player_by_mission_id(mission_id, reward_groups)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mission_id** | **String** |  | [required] |
**reward_groups** | [**LolMissionsRewardGroupsSelection**](LolMissionsRewardGroupsSelection.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_missions_v2_player_opt

> put_lol_missions_v2_player_opt(series_opt)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_opt** | [**LolMissionsSeriesOpt**](LolMissionsSeriesOpt.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

