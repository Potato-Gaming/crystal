# \PluginLolGameClientChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_game_client_chat_v1_buddies**](PluginLolGameClientChatApi.md#get_lol_game_client_chat_v1_buddies) | **Get** /lol-game-client-chat/v1/buddies | 
[**get_lol_game_client_chat_v1_ignored_summoners**](PluginLolGameClientChatApi.md#get_lol_game_client_chat_v1_ignored_summoners) | **Get** /lol-game-client-chat/v1/ignored-summoners | 
[**get_lol_game_client_chat_v1_muted_summoners**](PluginLolGameClientChatApi.md#get_lol_game_client_chat_v1_muted_summoners) | **Get** /lol-game-client-chat/v1/muted-summoners | 
[**post_lol_game_client_chat_v1_instant_messages**](PluginLolGameClientChatApi.md#post_lol_game_client_chat_v1_instant_messages) | **Post** /lol-game-client-chat/v1/instant-messages | 
[**post_lol_game_client_chat_v1_party_messages**](PluginLolGameClientChatApi.md#post_lol_game_client_chat_v1_party_messages) | **Post** /lol-game-client-chat/v1/party-messages | 



## get_lol_game_client_chat_v1_buddies

> Vec<String> get_lol_game_client_chat_v1_buddies()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_game_client_chat_v1_ignored_summoners

> Vec<i64> get_lol_game_client_chat_v1_ignored_summoners()


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


## get_lol_game_client_chat_v1_muted_summoners

> Vec<i64> get_lol_game_client_chat_v1_muted_summoners()


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


## post_lol_game_client_chat_v1_instant_messages

> post_lol_game_client_chat_v1_instant_messages(summoner_name, message)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_name** | **String** |  | [required] |
**message** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_game_client_chat_v1_party_messages

> post_lol_game_client_chat_v1_party_messages(message)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

