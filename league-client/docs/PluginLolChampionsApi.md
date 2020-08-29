# \PluginLolChampionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_champions_v1_inventories_by_summoner_id_champions**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions) | **Get** /lol-champions/v1/inventories/{summonerId}/champions | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id) | **Get** /lol-champions/v1/inventories/{summonerId}/champions/{championId} | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins) | **Get** /lol-champions/v1/inventories/{summonerId}/champions/{championId}/skins | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id) | **Get** /lol-champions/v1/inventories/{summonerId}/champions/{championId}/skins/{championSkinId} | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas) | **Get** /lol-champions/v1/inventories/{summonerId}/champions/{championId}/skins/{skinId}/chromas | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_minimal**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_minimal) | **Get** /lol-champions/v1/inventories/{summonerId}/champions-minimal | 
[**get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count) | **Get** /lol-champions/v1/inventories/{summonerId}/champions-playable-count | 
[**get_lol_champions_v1_inventories_by_summoner_id_skins_minimal**](PluginLolChampionsApi.md#get_lol_champions_v1_inventories_by_summoner_id_skins_minimal) | **Get** /lol-champions/v1/inventories/{summonerId}/skins-minimal | 
[**get_lol_champions_v1_owned_champions_minimal**](PluginLolChampionsApi.md#get_lol_champions_v1_owned_champions_minimal) | **Get** /lol-champions/v1/owned-champions-minimal | 



## get_lol_champions_v1_inventories_by_summoner_id_champions

> Vec<crate::models::LolChampionsCollectionsChampion> get_lol_champions_v1_inventories_by_summoner_id_champions(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampion>**](LolChampionsCollectionsChampion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id

> crate::models::LolChampionsCollectionsChampion get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id(summoner_id, champion_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**champion_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolChampionsCollectionsChampion**](LolChampionsCollectionsChampion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins

> Vec<crate::models::LolChampionsCollectionsChampionSkin> get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins(summoner_id, champion_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**champion_id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampionSkin>**](LolChampionsCollectionsChampionSkin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id

> crate::models::LolChampionsCollectionsChampionSkin get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id(summoner_id, champion_id, champion_skin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**champion_id** | **i32** |  | [required] |
**champion_skin_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolChampionsCollectionsChampionSkin**](LolChampionsCollectionsChampionSkin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas

> Vec<crate::models::LolChampionsCollectionsChampionChroma> get_lol_champions_v1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas(summoner_id, champion_id, skin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**champion_id** | **i32** |  | [required] |
**skin_id** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampionChroma>**](LolChampionsCollectionsChampionChroma.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_minimal

> Vec<crate::models::LolChampionsCollectionsChampionMinimal> get_lol_champions_v1_inventories_by_summoner_id_champions_minimal(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampionMinimal>**](LolChampionsCollectionsChampionMinimal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count

> crate::models::LolChampionsCollectionsChampionPlayableCounts get_lol_champions_v1_inventories_by_summoner_id_champions_playable_count(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolChampionsCollectionsChampionPlayableCounts**](LolChampionsCollectionsChampionPlayableCounts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_inventories_by_summoner_id_skins_minimal

> Vec<crate::models::LolChampionsCollectionsChampionSkinMinimal> get_lol_champions_v1_inventories_by_summoner_id_skins_minimal(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampionSkinMinimal>**](LolChampionsCollectionsChampionSkinMinimal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champions_v1_owned_champions_minimal

> Vec<crate::models::LolChampionsCollectionsChampionMinimal> get_lol_champions_v1_owned_champions_minimal()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolChampionsCollectionsChampionMinimal>**](LolChampionsCollectionsChampionMinimal.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

