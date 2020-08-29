# \PluginLolChampSelectLegacyApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_champ_select_legacy_v1_bannable_champion_ids**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_bannable_champion_ids) | **Get** /lol-champ-select-legacy/v1/bannable-champion-ids | 
[**get_lol_champ_select_legacy_v1_current_champion**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_current_champion) | **Get** /lol-champ-select-legacy/v1/current-champion | 
[**get_lol_champ_select_legacy_v1_disabled_champion_ids**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_disabled_champion_ids) | **Get** /lol-champ-select-legacy/v1/disabled-champion-ids | 
[**get_lol_champ_select_legacy_v1_implementation_active**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_implementation_active) | **Get** /lol-champ-select-legacy/v1/implementation-active | 
[**get_lol_champ_select_legacy_v1_pickable_champion_ids**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_pickable_champion_ids) | **Get** /lol-champ-select-legacy/v1/pickable-champion-ids | 
[**get_lol_champ_select_legacy_v1_session**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_session) | **Get** /lol-champ-select-legacy/v1/session | 
[**get_lol_champ_select_legacy_v1_session_timer**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_session_timer) | **Get** /lol-champ-select-legacy/v1/session/timer | 
[**get_lol_champ_select_legacy_v1_session_trades**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_session_trades) | **Get** /lol-champ-select-legacy/v1/session/trades | 
[**get_lol_champ_select_legacy_v1_session_trades_by_id**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_session_trades_by_id) | **Get** /lol-champ-select-legacy/v1/session/trades/{id} | 
[**get_lol_champ_select_legacy_v1_team_boost**](PluginLolChampSelectLegacyApi.md#get_lol_champ_select_legacy_v1_team_boost) | **Get** /lol-champ-select-legacy/v1/team-boost | 
[**patch_lol_champ_select_legacy_v1_session_actions_by_id**](PluginLolChampSelectLegacyApi.md#patch_lol_champ_select_legacy_v1_session_actions_by_id) | **Patch** /lol-champ-select-legacy/v1/session/actions/{id} | 
[**patch_lol_champ_select_legacy_v1_session_my_selection**](PluginLolChampSelectLegacyApi.md#patch_lol_champ_select_legacy_v1_session_my_selection) | **Patch** /lol-champ-select-legacy/v1/session/my-selection | 
[**post_lol_champ_select_legacy_v1_battle_training_launch**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_battle_training_launch) | **Post** /lol-champ-select-legacy/v1/battle-training/launch | 
[**post_lol_champ_select_legacy_v1_session_actions_by_id_complete**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_actions_by_id_complete) | **Post** /lol-champ-select-legacy/v1/session/actions/{id}/complete | 
[**post_lol_champ_select_legacy_v1_session_my_selection_reroll**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_my_selection_reroll) | **Post** /lol-champ-select-legacy/v1/session/my-selection/reroll | 
[**post_lol_champ_select_legacy_v1_session_trades_by_id_accept**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_trades_by_id_accept) | **Post** /lol-champ-select-legacy/v1/session/trades/{id}/accept | 
[**post_lol_champ_select_legacy_v1_session_trades_by_id_cancel**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_trades_by_id_cancel) | **Post** /lol-champ-select-legacy/v1/session/trades/{id}/cancel | 
[**post_lol_champ_select_legacy_v1_session_trades_by_id_decline**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_trades_by_id_decline) | **Post** /lol-champ-select-legacy/v1/session/trades/{id}/decline | 
[**post_lol_champ_select_legacy_v1_session_trades_by_id_request**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_session_trades_by_id_request) | **Post** /lol-champ-select-legacy/v1/session/trades/{id}/request | 
[**post_lol_champ_select_legacy_v1_team_boost_purchase**](PluginLolChampSelectLegacyApi.md#post_lol_champ_select_legacy_v1_team_boost_purchase) | **Post** /lol-champ-select-legacy/v1/team-boost/purchase | 



## get_lol_champ_select_legacy_v1_bannable_champion_ids

> Vec<i32> get_lol_champ_select_legacy_v1_bannable_champion_ids()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_current_champion

> i32 get_lol_champ_select_legacy_v1_current_champion()


### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_disabled_champion_ids

> Vec<i32> get_lol_champ_select_legacy_v1_disabled_champion_ids()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_implementation_active

> bool get_lol_champ_select_legacy_v1_implementation_active()


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


## get_lol_champ_select_legacy_v1_pickable_champion_ids

> Vec<i32> get_lol_champ_select_legacy_v1_pickable_champion_ids()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_session

> crate::models::LolChampSelectLegacyChampSelectSession get_lol_champ_select_legacy_v1_session()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolChampSelectLegacyChampSelectSession**](LolChampSelectLegacyChampSelectSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_session_timer

> crate::models::LolChampSelectLegacyChampSelectTimer get_lol_champ_select_legacy_v1_session_timer()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolChampSelectLegacyChampSelectTimer**](LolChampSelectLegacyChampSelectTimer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_session_trades

> Vec<crate::models::LolChampSelectLegacyChampSelectTradeContract> get_lol_champ_select_legacy_v1_session_trades()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolChampSelectLegacyChampSelectTradeContract>**](LolChampSelectLegacyChampSelectTradeContract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_session_trades_by_id

> crate::models::LolChampSelectLegacyChampSelectTradeContract get_lol_champ_select_legacy_v1_session_trades_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolChampSelectLegacyChampSelectTradeContract**](LolChampSelectLegacyChampSelectTradeContract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_champ_select_legacy_v1_team_boost

> crate::models::LolChampSelectLegacyTeamBoost get_lol_champ_select_legacy_v1_team_boost()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolChampSelectLegacyTeamBoost**](LolChampSelectLegacyTeamBoost.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_champ_select_legacy_v1_session_actions_by_id

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_champ_select_legacy_v1_session_actions_by_id(id, data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |
**data** | [**LolChampSelectLegacyChampSelectAction**](LolChampSelectLegacyChampSelectAction.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_champ_select_legacy_v1_session_my_selection

> ::std::collections::HashMap<String, serde_json::Value> patch_lol_champ_select_legacy_v1_session_my_selection(selection)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**selection** | [**LolChampSelectLegacyChampSelectMySelection**](LolChampSelectLegacyChampSelectMySelection.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_battle_training_launch

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_battle_training_launch()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_actions_by_id_complete

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_session_actions_by_id_complete(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_my_selection_reroll

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_session_my_selection_reroll()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_trades_by_id_accept

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_session_trades_by_id_accept(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_trades_by_id_cancel

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_session_trades_by_id_cancel(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_trades_by_id_decline

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_session_trades_by_id_decline(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_session_trades_by_id_request

> crate::models::LolChampSelectLegacyChampSelectTradeContract post_lol_champ_select_legacy_v1_session_trades_by_id_request(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** |  | [required] |

### Return type

[**crate::models::LolChampSelectLegacyChampSelectTradeContract**](LolChampSelectLegacyChampSelectTradeContract.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_champ_select_legacy_v1_team_boost_purchase

> ::std::collections::HashMap<String, serde_json::Value> post_lol_champ_select_legacy_v1_team_boost_purchase()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

