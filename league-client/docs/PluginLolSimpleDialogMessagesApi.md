# \PluginLolSimpleDialogMessagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_simple_dialog_messages_v1_messages_by_message_id**](PluginLolSimpleDialogMessagesApi.md#delete_lol_simple_dialog_messages_v1_messages_by_message_id) | **Delete** /lol-simple-dialog-messages/v1/messages/{messageId} | 
[**get_lol_simple_dialog_messages_v1_messages**](PluginLolSimpleDialogMessagesApi.md#get_lol_simple_dialog_messages_v1_messages) | **Get** /lol-simple-dialog-messages/v1/messages | 
[**post_lol_simple_dialog_messages_v1_messages**](PluginLolSimpleDialogMessagesApi.md#post_lol_simple_dialog_messages_v1_messages) | **Post** /lol-simple-dialog-messages/v1/messages | 



## delete_lol_simple_dialog_messages_v1_messages_by_message_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_simple_dialog_messages_v1_messages_by_message_id(message_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_simple_dialog_messages_v1_messages

> Vec<crate::models::LolSimpleDialogMessagesMessage> get_lol_simple_dialog_messages_v1_messages()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolSimpleDialogMessagesMessage>**](LolSimpleDialogMessagesMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_simple_dialog_messages_v1_messages

> ::std::collections::HashMap<String, serde_json::Value> post_lol_simple_dialog_messages_v1_messages(message_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_request** | [**LolSimpleDialogMessagesLocalMessageRequest**](LolSimpleDialogMessagesLocalMessageRequest.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

