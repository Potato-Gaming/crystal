# \PluginLolPftApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_pft_v2_survey**](PluginLolPftApi.md#get_lol_pft_v2_survey) | **Get** /lol-pft/v2/survey | 
[**post_lol_pft_v2_events**](PluginLolPftApi.md#post_lol_pft_v2_events) | **Post** /lol-pft/v2/events | 
[**post_lol_pft_v2_survey**](PluginLolPftApi.md#post_lol_pft_v2_survey) | **Post** /lol-pft/v2/survey | 



## get_lol_pft_v2_survey

> crate::models::LolPftPftSurvey get_lol_pft_v2_survey()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPftPftSurvey**](LolPftPFTSurvey.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_pft_v2_events

> ::std::collections::HashMap<String, serde_json::Value> post_lol_pft_v2_events(pft_event)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pft_event** | [**LolPftPftEvent**](LolPftPftEvent.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_pft_v2_survey

> post_lol_pft_v2_survey(survey)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**survey** | [**LolPftPftSurvey**](LolPftPftSurvey.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

