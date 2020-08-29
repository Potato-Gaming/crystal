# \PluginLolPersonalizedOffersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_personalized_offers_v1_offers**](PluginLolPersonalizedOffersApi.md#get_lol_personalized_offers_v1_offers) | **Get** /lol-personalized-offers/v1/offers | 
[**get_lol_personalized_offers_v1_status**](PluginLolPersonalizedOffersApi.md#get_lol_personalized_offers_v1_status) | **Get** /lol-personalized-offers/v1/status | 
[**get_lol_personalized_offers_v1_themed**](PluginLolPersonalizedOffersApi.md#get_lol_personalized_offers_v1_themed) | **Get** /lol-personalized-offers/v1/themed | 
[**post_lol_personalized_offers_v1_offers_by_id_purchase**](PluginLolPersonalizedOffersApi.md#post_lol_personalized_offers_v1_offers_by_id_purchase) | **Post** /lol-personalized-offers/v1/offers/{id}/purchase | 
[**post_lol_personalized_offers_v1_offers_by_id_reveal**](PluginLolPersonalizedOffersApi.md#post_lol_personalized_offers_v1_offers_by_id_reveal) | **Post** /lol-personalized-offers/v1/offers/{id}/reveal | 
[**post_lol_personalized_offers_v1_offers_purchase**](PluginLolPersonalizedOffersApi.md#post_lol_personalized_offers_v1_offers_purchase) | **Post** /lol-personalized-offers/v1/offers/purchase | 
[**post_lol_personalized_offers_v1_offers_reveal**](PluginLolPersonalizedOffersApi.md#post_lol_personalized_offers_v1_offers_reveal) | **Post** /lol-personalized-offers/v1/offers/reveal | 



## get_lol_personalized_offers_v1_offers

> Vec<crate::models::LolPersonalizedOffersUiOffer> get_lol_personalized_offers_v1_offers()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPersonalizedOffersUiOffer>**](LolPersonalizedOffersUIOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_personalized_offers_v1_status

> crate::models::LolPersonalizedOffersUiStatus get_lol_personalized_offers_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPersonalizedOffersUiStatus**](LolPersonalizedOffersUIStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_personalized_offers_v1_themed

> bool get_lol_personalized_offers_v1_themed()


### Parameters

This endpoint does not need any parameter.

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_personalized_offers_v1_offers_by_id_purchase

> crate::models::LolPersonalizedOffersPurchaseResponse post_lol_personalized_offers_v1_offers_by_id_purchase(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::LolPersonalizedOffersPurchaseResponse**](LolPersonalizedOffersPurchaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_personalized_offers_v1_offers_by_id_reveal

> Vec<crate::models::LolPersonalizedOffersUiOffer> post_lol_personalized_offers_v1_offers_by_id_reveal(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolPersonalizedOffersUiOffer>**](LolPersonalizedOffersUIOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_personalized_offers_v1_offers_purchase

> crate::models::LolPersonalizedOffersPurchaseResponse post_lol_personalized_offers_v1_offers_purchase(offer_requests)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offer_requests** | [**LolPersonalizedOffersOfferRequests**](LolPersonalizedOffersOfferRequests.md) |  | [required] |

### Return type

[**crate::models::LolPersonalizedOffersPurchaseResponse**](LolPersonalizedOffersPurchaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_personalized_offers_v1_offers_reveal

> Vec<crate::models::LolPersonalizedOffersUiOffer> post_lol_personalized_offers_v1_offers_reveal(offer_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offer_ids** | [**LolPersonalizedOffersOfferIds**](LolPersonalizedOffersOfferIds.md) |  | [required] |

### Return type

[**Vec<crate::models::LolPersonalizedOffersUiOffer>**](LolPersonalizedOffersUIOffer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

