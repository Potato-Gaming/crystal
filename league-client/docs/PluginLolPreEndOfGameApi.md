# \PluginLolPreEndOfGameApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_pre_end_of_game_v1_registration_by_sequence_event_name**](PluginLolPreEndOfGameApi.md#delete_lol_pre_end_of_game_v1_registration_by_sequence_event_name) | **Delete** /lol-pre-end-of-game/v1/registration/{sequenceEventName} | 
[**get_lol_pre_end_of_game_v1_current_sequence_event**](PluginLolPreEndOfGameApi.md#get_lol_pre_end_of_game_v1_current_sequence_event) | **Get** /lol-pre-end-of-game/v1/currentSequenceEvent | 
[**post_lol_pre_end_of_game_v1_complete_by_sequence_event_name**](PluginLolPreEndOfGameApi.md#post_lol_pre_end_of_game_v1_complete_by_sequence_event_name) | **Post** /lol-pre-end-of-game/v1/complete/{sequenceEventName} | 
[**post_lol_pre_end_of_game_v1_registration_by_sequence_event_name_by_priority**](PluginLolPreEndOfGameApi.md#post_lol_pre_end_of_game_v1_registration_by_sequence_event_name_by_priority) | **Post** /lol-pre-end-of-game/v1/registration/{sequenceEventName}/{priority} | 



## delete_lol_pre_end_of_game_v1_registration_by_sequence_event_name

> delete_lol_pre_end_of_game_v1_registration_by_sequence_event_name(sequence_event_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_event_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_pre_end_of_game_v1_current_sequence_event

> crate::models::LolPreEndOfGameSequenceEvent get_lol_pre_end_of_game_v1_current_sequence_event()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPreEndOfGameSequenceEvent**](LolPreEndOfGameSequenceEvent.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_pre_end_of_game_v1_complete_by_sequence_event_name

> post_lol_pre_end_of_game_v1_complete_by_sequence_event_name(sequence_event_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_event_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_pre_end_of_game_v1_registration_by_sequence_event_name_by_priority

> post_lol_pre_end_of_game_v1_registration_by_sequence_event_name_by_priority(sequence_event_name, priority)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sequence_event_name** | **String** |  | [required] |
**priority** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

