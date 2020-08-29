# \PluginLolEmailVerificationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_email_verification_v1_email**](PluginLolEmailVerificationApi.md#get_lol_email_verification_v1_email) | **Get** /lol-email-verification/v1/email | 
[**post_lol_email_verification_v1_confirm_email**](PluginLolEmailVerificationApi.md#post_lol_email_verification_v1_confirm_email) | **Post** /lol-email-verification/v1/confirm-email | 
[**put_lol_email_verification_v1_email**](PluginLolEmailVerificationApi.md#put_lol_email_verification_v1_email) | **Put** /lol-email-verification/v1/email | 



## get_lol_email_verification_v1_email

> crate::models::LolEmailVerificationEmailVerificationSession get_lol_email_verification_v1_email()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolEmailVerificationEmailVerificationSession**](LolEmailVerificationEmailVerificationSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_email_verification_v1_confirm_email

> ::std::collections::HashMap<String, serde_json::Value> post_lol_email_verification_v1_confirm_email()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_email_verification_v1_email

> ::std::collections::HashMap<String, serde_json::Value> put_lol_email_verification_v1_email(email_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_update** | [**LolEmailVerificationEmailUpdate**](LolEmailVerificationEmailUpdate.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

