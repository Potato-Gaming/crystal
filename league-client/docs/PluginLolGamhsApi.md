# \PluginLolGamhsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_gamhs_v1_products_by_product_id_by_puuid_matches**](PluginLolGamhsApi.md#get_lol_gamhs_v1_products_by_product_id_by_puuid_matches) | **Get** /lol-gamhs/v1/products/{productId}/{puuid}/matches | 
[**get_lol_gamhs_v1_products_by_product_id_current_summoner_matches**](PluginLolGamhsApi.md#get_lol_gamhs_v1_products_by_product_id_current_summoner_matches) | **Get** /lol-gamhs/v1/products/{productId}/current-summoner/matches | 
[**get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_details**](PluginLolGamhsApi.md#get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_details) | **Get** /lol-gamhs/v1/products/{productId}/matches/{matchId}/details | 
[**get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_summary**](PluginLolGamhsApi.md#get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_summary) | **Get** /lol-gamhs/v1/products/{productId}/matches/{matchId}/summary | 



## get_lol_gamhs_v1_products_by_product_id_by_puuid_matches

> crate::models::LolGamhsMatchHistoryList get_lol_gamhs_v1_products_by_product_id_by_puuid_matches(product_id, puuid, begin, count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**puuid** | **String** |  | [required] |
**begin** | **i32** |  | [required] |
**count** | **i32** |  | [required] |

### Return type

[**crate::models::LolGamhsMatchHistoryList**](LolGamhsMatchHistoryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_gamhs_v1_products_by_product_id_current_summoner_matches

> crate::models::LolGamhsMatchHistoryList get_lol_gamhs_v1_products_by_product_id_current_summoner_matches(product_id, begin, count)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**begin** | **i32** |  | [required] |
**count** | **i32** |  | [required] |

### Return type

[**crate::models::LolGamhsMatchHistoryList**](LolGamhsMatchHistoryList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_details

> crate::models::LolGamhsMatchHistoryData get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_details(product_id, match_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**match_id** | **String** |  | [required] |

### Return type

[**crate::models::LolGamhsMatchHistoryData**](LolGamhsMatchHistoryData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_summary

> crate::models::LolGamhsMatchHistoryData get_lol_gamhs_v1_products_by_product_id_matches_by_match_id_summary(product_id, match_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |
**match_id** | **String** |  | [required] |

### Return type

[**crate::models::LolGamhsMatchHistoryData**](LolGamhsMatchHistoryData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

