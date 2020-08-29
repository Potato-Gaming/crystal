# \CookieJarApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cookie_jar_v1_cookies**](CookieJarApi.md#get_cookie_jar_v1_cookies) | **Get** /cookie-jar/v1/cookies | Get all cookies.
[**post_cookie_jar_v1_cookies**](CookieJarApi.md#post_cookie_jar_v1_cookies) | **Post** /cookie-jar/v1/cookies | Set a cookie.



## get_cookie_jar_v1_cookies

> Vec<crate::models::Cookie> get_cookie_jar_v1_cookies()
Get all cookies.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Cookie>**](cookie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_cookie_jar_v1_cookies

> ::std::collections::HashMap<String, serde_json::Value> post_cookie_jar_v1_cookies(cookie)
Set a cookie.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cookie** | [**Vec<crate::models::Cookie>**](cookie.md) | Cookie to set | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

