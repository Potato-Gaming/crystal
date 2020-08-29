# \PluginLolAccountVerificationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_account_verification_v1_device**](PluginLolAccountVerificationApi.md#get_lol_account_verification_v1_device) | **Get** /lol-account-verification/v1/device | 
[**get_lol_account_verification_v1_is_verified**](PluginLolAccountVerificationApi.md#get_lol_account_verification_v1_is_verified) | **Get** /lol-account-verification/v1/is-verified | 
[**post_lol_account_verification_v1_authenticate**](PluginLolAccountVerificationApi.md#post_lol_account_verification_v1_authenticate) | **Post** /lol-account-verification/v1/authenticate | 
[**post_lol_account_verification_v1_invalidate**](PluginLolAccountVerificationApi.md#post_lol_account_verification_v1_invalidate) | **Post** /lol-account-verification/v1/invalidate | 
[**post_lol_account_verification_v1_send_token**](PluginLolAccountVerificationApi.md#post_lol_account_verification_v1_send_token) | **Post** /lol-account-verification/v1/send-token | 
[**post_lol_account_verification_v1_verify**](PluginLolAccountVerificationApi.md#post_lol_account_verification_v1_verify) | **Post** /lol-account-verification/v1/verify | 



## get_lol_account_verification_v1_device

> crate::models::LolAccountVerificationDeviceResponse get_lol_account_verification_v1_device()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolAccountVerificationDeviceResponse**](LolAccountVerificationDeviceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_account_verification_v1_is_verified

> crate::models::LolAccountVerificationIsVerifiedResponse get_lol_account_verification_v1_is_verified()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolAccountVerificationIsVerifiedResponse**](LolAccountVerificationIsVerifiedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_account_verification_v1_authenticate

> crate::models::LolAccountVerificationAuthenticateResponse post_lol_account_verification_v1_authenticate(authenticate_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authenticate_request** | [**LolAccountVerificationAuthenticateRequest**](LolAccountVerificationAuthenticateRequest.md) |  | [required] |

### Return type

[**crate::models::LolAccountVerificationAuthenticateResponse**](LolAccountVerificationAuthenticateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_account_verification_v1_invalidate

> crate::models::LolAccountVerificationInvalidateResponse post_lol_account_verification_v1_invalidate()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolAccountVerificationInvalidateResponse**](LolAccountVerificationInvalidateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_account_verification_v1_send_token

> crate::models::LolAccountVerificationSendTokenResponse post_lol_account_verification_v1_send_token(send_token_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_token_request** | [**LolAccountVerificationSendTokenRequest**](LolAccountVerificationSendTokenRequest.md) |  | [required] |

### Return type

[**crate::models::LolAccountVerificationSendTokenResponse**](LolAccountVerificationSendTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_account_verification_v1_verify

> crate::models::LolAccountVerificationVerifyResponse post_lol_account_verification_v1_verify(verify_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_request** | [**LolAccountVerificationVerifyRequest**](LolAccountVerificationVerifyRequest.md) |  | [required] |

### Return type

[**crate::models::LolAccountVerificationVerifyResponse**](LolAccountVerificationVerifyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

