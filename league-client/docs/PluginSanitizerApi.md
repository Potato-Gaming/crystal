# \PluginSanitizerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sanitizer_v1_status**](PluginSanitizerApi.md#get_sanitizer_v1_status) | **Get** /sanitizer/v1/status | 
[**post_sanitizer_v1_contains_sanitized**](PluginSanitizerApi.md#post_sanitizer_v1_contains_sanitized) | **Post** /sanitizer/v1/containsSanitized | 
[**post_sanitizer_v1_sanitize**](PluginSanitizerApi.md#post_sanitizer_v1_sanitize) | **Post** /sanitizer/v1/sanitize | 



## get_sanitizer_v1_status

> crate::models::SanitizerSanitizerStatus get_sanitizer_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SanitizerSanitizerStatus**](SanitizerSanitizerStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sanitizer_v1_contains_sanitized

> crate::models::SanitizerContainsSanitizedResponse post_sanitizer_v1_contains_sanitized(request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**SanitizerContainsSanitizedRequest**](SanitizerContainsSanitizedRequest.md) |  | [required] |

### Return type

[**crate::models::SanitizerContainsSanitizedResponse**](SanitizerContainsSanitizedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_sanitizer_v1_sanitize

> crate::models::SanitizerSanitizeResponse post_sanitizer_v1_sanitize(request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request** | [**SanitizerSanitizeRequest**](SanitizerSanitizeRequest.md) |  | [required] |

### Return type

[**crate::models::SanitizerSanitizeResponse**](SanitizerSanitizeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

