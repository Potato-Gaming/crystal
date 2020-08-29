# \PluginLolCollectionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_collections_v1_inventories_by_summoner_id_backdrop**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_backdrop) | **Get** /lol-collections/v1/inventories/{summonerId}/backdrop | 
[**get_lol_collections_v1_inventories_by_summoner_id_champion_mastery**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_champion_mastery) | **Get** /lol-collections/v1/inventories/{summonerId}/champion-mastery | 
[**get_lol_collections_v1_inventories_by_summoner_id_champion_mastery_top**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_champion_mastery_top) | **Get** /lol-collections/v1/inventories/{summonerId}/champion-mastery/top | 
[**get_lol_collections_v1_inventories_by_summoner_id_spells**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_spells) | **Get** /lol-collections/v1/inventories/{summonerId}/spells | 
[**get_lol_collections_v1_inventories_by_summoner_id_summoner_icons**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_summoner_icons) | **Get** /lol-collections/v1/inventories/{summonerId}/summoner-icons | 
[**get_lol_collections_v1_inventories_by_summoner_id_ward_skins**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_ward_skins) | **Get** /lol-collections/v1/inventories/{summonerId}/ward-skins | 
[**get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id) | **Get** /lol-collections/v1/inventories/{summonerId}/ward-skins/{wardSkinId} | 
[**get_lol_collections_v1_inventories_chest_eligibility**](PluginLolCollectionsApi.md#get_lol_collections_v1_inventories_chest_eligibility) | **Get** /lol-collections/v1/inventories/chest-eligibility | 
[**get_lol_collections_v2_inventories_by_summoner_id_summoner_icons**](PluginLolCollectionsApi.md#get_lol_collections_v2_inventories_by_summoner_id_summoner_icons) | **Get** /lol-collections/v2/inventories/{summonerId}/summoner-icons | 
[**get_lol_collections_v2_inventories_by_summoner_id_summoner_icons_by_summoner_icon_id**](PluginLolCollectionsApi.md#get_lol_collections_v2_inventories_by_summoner_id_summoner_icons_by_summoner_icon_id) | **Get** /lol-collections/v2/inventories/{summonerId}/summoner-icons/{summonerIconId} | 
[**put_lol_collections_v1_inventories_by_summoner_id_verification**](PluginLolCollectionsApi.md#put_lol_collections_v1_inventories_by_summoner_id_verification) | **Put** /lol-collections/v1/inventories/{summonerId}/verification | 



## get_lol_collections_v1_inventories_by_summoner_id_backdrop

> crate::models::LolCollectionsCollectionsSummonerBackdrop get_lol_collections_v1_inventories_by_summoner_id_backdrop(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsSummonerBackdrop**](LolCollectionsCollectionsSummonerBackdrop.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_champion_mastery

> Vec<crate::models::LolCollectionsCollectionsChampionMastery> get_lol_collections_v1_inventories_by_summoner_id_champion_mastery(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**Vec<crate::models::LolCollectionsCollectionsChampionMastery>**](LolCollectionsCollectionsChampionMastery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_champion_mastery_top

> crate::models::LolCollectionsCollectionsTopChampionMasteries get_lol_collections_v1_inventories_by_summoner_id_champion_mastery_top(summoner_id, limit, sort_rule)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**limit** | **i64** |  | [required] |
**sort_rule** | Option<**String**> |  |  |

### Return type

[**crate::models::LolCollectionsCollectionsTopChampionMasteries**](LolCollectionsCollectionsTopChampionMasteries.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_spells

> crate::models::LolCollectionsCollectionsSummonerSpells get_lol_collections_v1_inventories_by_summoner_id_spells(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsSummonerSpells**](LolCollectionsCollectionsSummonerSpells.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_summoner_icons

> crate::models::LolCollectionsCollectionsSummonerIcons get_lol_collections_v1_inventories_by_summoner_id_summoner_icons(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsSummonerIcons**](LolCollectionsCollectionsSummonerIcons.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_ward_skins

> Vec<crate::models::LolCollectionsCollectionsWardSkin> get_lol_collections_v1_inventories_by_summoner_id_ward_skins(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**Vec<crate::models::LolCollectionsCollectionsWardSkin>**](LolCollectionsCollectionsWardSkin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id

> crate::models::LolCollectionsCollectionsWardSkin get_lol_collections_v1_inventories_by_summoner_id_ward_skins_by_ward_skin_id(summoner_id, ward_skin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**ward_skin_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsWardSkin**](LolCollectionsCollectionsWardSkin.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v1_inventories_chest_eligibility

> crate::models::LolCollectionsCollectionsChestEligibility get_lol_collections_v1_inventories_chest_eligibility()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolCollectionsCollectionsChestEligibility**](LolCollectionsCollectionsChestEligibility.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v2_inventories_by_summoner_id_summoner_icons

> crate::models::LolCollectionsCollectionsSummonerIcons get_lol_collections_v2_inventories_by_summoner_id_summoner_icons(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsSummonerIcons**](LolCollectionsCollectionsSummonerIcons.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_collections_v2_inventories_by_summoner_id_summoner_icons_by_summoner_icon_id

> crate::models::LolCollectionsCollectionsSummonerIcon get_lol_collections_v2_inventories_by_summoner_id_summoner_icons_by_summoner_icon_id(summoner_id, summoner_icon_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**summoner_icon_id** | **i32** |  | [required] |

### Return type

[**crate::models::LolCollectionsCollectionsSummonerIcon**](LolCollectionsCollectionsSummonerIcon.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_collections_v1_inventories_by_summoner_id_verification

> String put_lol_collections_v1_inventories_by_summoner_id_verification(summoner_id, verification_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | **i64** |  | [required] |
**verification_code** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

