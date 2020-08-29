# \PluginLolHonorV2Api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_honor_v2_v1_ballot**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_ballot) | **Get** /lol-honor-v2/v1/ballot | 
[**get_lol_honor_v2_v1_config**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_config) | **Get** /lol-honor-v2/v1/config | 
[**get_lol_honor_v2_v1_late_recognition**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_late_recognition) | **Get** /lol-honor-v2/v1/late-recognition | 
[**get_lol_honor_v2_v1_latest_eligible_game**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_latest_eligible_game) | **Get** /lol-honor-v2/v1/latest-eligible-game | 
[**get_lol_honor_v2_v1_level_change**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_level_change) | **Get** /lol-honor-v2/v1/level-change | 
[**get_lol_honor_v2_v1_mutual_honor**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_mutual_honor) | **Get** /lol-honor-v2/v1/mutual-honor | 
[**get_lol_honor_v2_v1_profile**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_profile) | **Get** /lol-honor-v2/v1/profile | 
[**get_lol_honor_v2_v1_recognition**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_recognition) | **Get** /lol-honor-v2/v1/recognition | 
[**get_lol_honor_v2_v1_reward_granted**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_reward_granted) | **Get** /lol-honor-v2/v1/reward-granted | 
[**get_lol_honor_v2_v1_team_choices**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_team_choices) | **Get** /lol-honor-v2/v1/team-choices | 
[**get_lol_honor_v2_v1_vote_completion**](PluginLolHonorV2Api.md#get_lol_honor_v2_v1_vote_completion) | **Get** /lol-honor-v2/v1/vote-completion | 
[**post_lol_honor_v2_v1_honor_player**](PluginLolHonorV2Api.md#post_lol_honor_v2_v1_honor_player) | **Post** /lol-honor-v2/v1/honor-player | 
[**post_lol_honor_v2_v1_late_recognition_ack**](PluginLolHonorV2Api.md#post_lol_honor_v2_v1_late_recognition_ack) | **Post** /lol-honor-v2/v1/late-recognition/ack | 
[**post_lol_honor_v2_v1_level_change_ack**](PluginLolHonorV2Api.md#post_lol_honor_v2_v1_level_change_ack) | **Post** /lol-honor-v2/v1/level-change/ack | 
[**post_lol_honor_v2_v1_mutual_honor_ack**](PluginLolHonorV2Api.md#post_lol_honor_v2_v1_mutual_honor_ack) | **Post** /lol-honor-v2/v1/mutual-honor/ack | 
[**post_lol_honor_v2_v1_reward_granted_ack**](PluginLolHonorV2Api.md#post_lol_honor_v2_v1_reward_granted_ack) | **Post** /lol-honor-v2/v1/reward-granted/ack | 



## get_lol_honor_v2_v1_ballot

> crate::models::LolHonorV2Ballot get_lol_honor_v2_v1_ballot()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2Ballot**](LolHonorV2Ballot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_config

> crate::models::LolHonorV2HonorConfig get_lol_honor_v2_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2HonorConfig**](LolHonorV2HonorConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_late_recognition

> Vec<crate::models::LolHonorV2Honor> get_lol_honor_v2_v1_late_recognition()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolHonorV2Honor>**](LolHonorV2Honor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_latest_eligible_game

> i64 get_lol_honor_v2_v1_latest_eligible_game()


### Parameters

This endpoint does not need any parameter.

### Return type

**i64**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_level_change

> crate::models::LolHonorV2VendedHonorChange get_lol_honor_v2_v1_level_change()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2VendedHonorChange**](LolHonorV2VendedHonorChange.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_mutual_honor

> crate::models::LolHonorV2MutualHonor get_lol_honor_v2_v1_mutual_honor()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2MutualHonor**](LolHonorV2MutualHonor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_profile

> crate::models::LolHonorV2ProfileInfo get_lol_honor_v2_v1_profile()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2ProfileInfo**](LolHonorV2ProfileInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_recognition

> Vec<crate::models::LolHonorV2Honor> get_lol_honor_v2_v1_recognition()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolHonorV2Honor>**](LolHonorV2Honor.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_reward_granted

> crate::models::LolHonorV2VendedReward get_lol_honor_v2_v1_reward_granted()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2VendedReward**](LolHonorV2VendedReward.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_honor_v2_v1_team_choices

> Vec<i64> get_lol_honor_v2_v1_team_choices()


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


## get_lol_honor_v2_v1_vote_completion

> crate::models::LolHonorV2VoteCompletion get_lol_honor_v2_v1_vote_completion()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolHonorV2VoteCompletion**](LolHonorV2VoteCompletion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_honor_v2_v1_honor_player

> String post_lol_honor_v2_v1_honor_player(honor_player_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**honor_player_request** | [**LolHonorV2ApiHonorPlayerServerRequest**](LolHonorV2ApiHonorPlayerServerRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_honor_v2_v1_late_recognition_ack

> post_lol_honor_v2_v1_late_recognition_ack()


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


## post_lol_honor_v2_v1_level_change_ack

> post_lol_honor_v2_v1_level_change_ack()


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


## post_lol_honor_v2_v1_mutual_honor_ack

> post_lol_honor_v2_v1_mutual_honor_ack()


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


## post_lol_honor_v2_v1_reward_granted_ack

> post_lol_honor_v2_v1_reward_granted_ack()


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

