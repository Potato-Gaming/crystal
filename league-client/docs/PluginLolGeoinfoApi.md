# \PluginLolGeoinfoApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_geoinfo_v1_getlocation**](PluginLolGeoinfoApi.md#get_lol_geoinfo_v1_getlocation) | **Get** /lol-geoinfo/v1/getlocation | 
[**get_lol_geoinfo_v1_whereami**](PluginLolGeoinfoApi.md#get_lol_geoinfo_v1_whereami) | **Get** /lol-geoinfo/v1/whereami | 



## get_lol_geoinfo_v1_getlocation

> crate::models::LolGeoinfoGeoInfo get_lol_geoinfo_v1_getlocation(ip_address)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ip_address** | **String** |  | [required] |

### Return type

[**crate::models::LolGeoinfoGeoInfo**](LolGeoinfoGeoInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_geoinfo_v1_whereami

> crate::models::LolGeoinfoGeoInfoResponse get_lol_geoinfo_v1_whereami()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolGeoinfoGeoInfoResponse**](LolGeoinfoGeoInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

