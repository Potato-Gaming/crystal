# \PluginLolPlayerBehaviorApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_player_behavior_v1_reporter_feedback_by_id**](PluginLolPlayerBehaviorApi.md#delete_lol_player_behavior_v1_reporter_feedback_by_id) | **Delete** /lol-player-behavior/v1/reporter-feedback/{id} | 
[**get_lol_player_behavior_v1_ban**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_ban) | **Get** /lol-player-behavior/v1/ban | 
[**get_lol_player_behavior_v1_chat_restriction**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_chat_restriction) | **Get** /lol-player-behavior/v1/chat-restriction | 
[**get_lol_player_behavior_v1_config**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_config) | **Get** /lol-player-behavior/v1/config | 
[**get_lol_player_behavior_v1_ranked_restriction**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_ranked_restriction) | **Get** /lol-player-behavior/v1/ranked-restriction | 
[**get_lol_player_behavior_v1_reform_card**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_reform_card) | **Get** /lol-player-behavior/v1/reform-card | 
[**get_lol_player_behavior_v1_reporter_feedback**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_reporter_feedback) | **Get** /lol-player-behavior/v1/reporter-feedback | 
[**get_lol_player_behavior_v1_reporter_feedback_by_id**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v1_reporter_feedback_by_id) | **Get** /lol-player-behavior/v1/reporter-feedback/{id} | 
[**get_lol_player_behavior_v2_reform_card**](PluginLolPlayerBehaviorApi.md#get_lol_player_behavior_v2_reform_card) | **Get** /lol-player-behavior/v2/reform-card | 



## delete_lol_player_behavior_v1_reporter_feedback_by_id

> crate::models::LolPlayerBehaviorReporterFeedback delete_lol_player_behavior_v1_reporter_feedback_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::LolPlayerBehaviorReporterFeedback**](LolPlayerBehaviorReporterFeedback.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_ban

> crate::models::LolPlayerBehaviorBanNotification get_lol_player_behavior_v1_ban()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorBanNotification**](LolPlayerBehaviorBanNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_chat_restriction

> crate::models::LolPlayerBehaviorRestrictionNotification get_lol_player_behavior_v1_chat_restriction()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorRestrictionNotification**](LolPlayerBehaviorRestrictionNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_config

> crate::models::LolPlayerBehaviorPlayerBehaviorConfig get_lol_player_behavior_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorPlayerBehaviorConfig**](LolPlayerBehaviorPlayerBehaviorConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_ranked_restriction

> crate::models::LolPlayerBehaviorRestrictionNotification get_lol_player_behavior_v1_ranked_restriction()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorRestrictionNotification**](LolPlayerBehaviorRestrictionNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_reform_card

> crate::models::LolPlayerBehaviorReformCard get_lol_player_behavior_v1_reform_card()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorReformCard**](LolPlayerBehaviorReformCard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_reporter_feedback

> Vec<crate::models::LolPlayerBehaviorReporterFeedback> get_lol_player_behavior_v1_reporter_feedback()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPlayerBehaviorReporterFeedback>**](LolPlayerBehaviorReporterFeedback.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v1_reporter_feedback_by_id

> crate::models::LolPlayerBehaviorReporterFeedback get_lol_player_behavior_v1_reporter_feedback_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::LolPlayerBehaviorReporterFeedback**](LolPlayerBehaviorReporterFeedback.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_player_behavior_v2_reform_card

> crate::models::LolPlayerBehaviorReformCardV2 get_lol_player_behavior_v2_reform_card()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPlayerBehaviorReformCardV2**](LolPlayerBehaviorReformCardV2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

