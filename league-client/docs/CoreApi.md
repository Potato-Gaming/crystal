# \CoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_riotclient_addorupdatemetric**](CoreApi.md#post_riotclient_addorupdatemetric) | **Post** /riotclient/addorupdatemetric | Adds or Updates a Metric



## post_riotclient_addorupdatemetric

> post_riotclient_addorupdatemetric(group, object, name, value)
Adds or Updates a Metric

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group** | **String** | Name of metric group | [required] |
**object** | **String** | Name of metric object | [required] |
**name** | **String** | Name of metric item | [required] |
**value** | **i64** | Value to store | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

