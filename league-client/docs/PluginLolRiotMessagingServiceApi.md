# \PluginLolRiotMessagingServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_rms_v1_champion_mastery_leaveup_update_by_id**](PluginLolRiotMessagingServiceApi.md#delete_lol_rms_v1_champion_mastery_leaveup_update_by_id) | **Delete** /lol-rms/v1/champion-mastery-leaveup-update/{id} | 
[**get_lol_rms_v1_champion_mastery_leaveup_update**](PluginLolRiotMessagingServiceApi.md#get_lol_rms_v1_champion_mastery_leaveup_update) | **Get** /lol-rms/v1/champion-mastery-leaveup-update | 



## delete_lol_rms_v1_champion_mastery_leaveup_update_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_rms_v1_champion_mastery_leaveup_update_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_rms_v1_champion_mastery_leaveup_update

> Vec<crate::models::LolRiotMessagingServiceChampionMasteryLevelUp> get_lol_rms_v1_champion_mastery_leaveup_update()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolRiotMessagingServiceChampionMasteryLevelUp>**](LolRiotMessagingServiceChampionMasteryLevelUp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

