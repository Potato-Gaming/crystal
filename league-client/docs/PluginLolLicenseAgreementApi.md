# \PluginLolLicenseAgreementApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_license_agreement_v1_agreements**](PluginLolLicenseAgreementApi.md#get_lol_license_agreement_v1_agreements) | **Get** /lol-license-agreement/v1/agreements | 
[**get_lol_license_agreement_v1_all_agreements**](PluginLolLicenseAgreementApi.md#get_lol_license_agreement_v1_all_agreements) | **Get** /lol-license-agreement/v1/all-agreements | 
[**get_lol_license_agreement_v1_serve_location**](PluginLolLicenseAgreementApi.md#get_lol_license_agreement_v1_serve_location) | **Get** /lol-license-agreement/v1/serve-location | 
[**post_lol_license_agreement_v1_agreements_by_id_accept**](PluginLolLicenseAgreementApi.md#post_lol_license_agreement_v1_agreements_by_id_accept) | **Post** /lol-license-agreement/v1/agreements/{id}/accept | 
[**post_lol_license_agreement_v1_agreements_by_id_decline**](PluginLolLicenseAgreementApi.md#post_lol_license_agreement_v1_agreements_by_id_decline) | **Post** /lol-license-agreement/v1/agreements/{id}/decline | 



## get_lol_license_agreement_v1_agreements

> Vec<crate::models::LolLicenseAgreementLicenseAgreement> get_lol_license_agreement_v1_agreements()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLicenseAgreementLicenseAgreement>**](LolLicenseAgreementLicenseAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_license_agreement_v1_all_agreements

> Vec<crate::models::LolLicenseAgreementLicenseAgreement> get_lol_license_agreement_v1_all_agreements()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLicenseAgreementLicenseAgreement>**](LolLicenseAgreementLicenseAgreement.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_license_agreement_v1_serve_location

> crate::models::LolLicenseAgreementLicenseServeLocation get_lol_license_agreement_v1_serve_location()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLicenseAgreementLicenseServeLocation**](LolLicenseAgreementLicenseServeLocation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_license_agreement_v1_agreements_by_id_accept

> ::std::collections::HashMap<String, serde_json::Value> post_lol_license_agreement_v1_agreements_by_id_accept(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_license_agreement_v1_agreements_by_id_decline

> ::std::collections::HashMap<String, serde_json::Value> post_lol_license_agreement_v1_agreements_by_id_decline(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

