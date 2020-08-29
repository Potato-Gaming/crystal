# \PluginLolLootApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_loot_v1_loot_grants_by_id**](PluginLolLootApi.md#delete_lol_loot_v1_loot_grants_by_id) | **Delete** /lol-loot/v1/loot-grants/{id} | 
[**delete_lol_loot_v1_player_loot_by_loot_id_new_notification**](PluginLolLootApi.md#delete_lol_loot_v1_player_loot_by_loot_id_new_notification) | **Delete** /lol-loot/v1/player-loot/{lootId}/new-notification | 
[**get_lol_loot_v1_currency_configuration**](PluginLolLootApi.md#get_lol_loot_v1_currency_configuration) | **Get** /lol-loot/v1/currency-configuration | 
[**get_lol_loot_v1_enabled**](PluginLolLootApi.md#get_lol_loot_v1_enabled) | **Get** /lol-loot/v1/enabled | 
[**get_lol_loot_v1_loot_grants**](PluginLolLootApi.md#get_lol_loot_v1_loot_grants) | **Get** /lol-loot/v1/loot-grants | 
[**get_lol_loot_v1_loot_items**](PluginLolLootApi.md#get_lol_loot_v1_loot_items) | **Get** /lol-loot/v1/loot-items | 
[**get_lol_loot_v1_new_player_check_done**](PluginLolLootApi.md#get_lol_loot_v1_new_player_check_done) | **Get** /lol-loot/v1/new-player-check-done | 
[**get_lol_loot_v1_player_display_categories**](PluginLolLootApi.md#get_lol_loot_v1_player_display_categories) | **Get** /lol-loot/v1/player-display-categories | 
[**get_lol_loot_v1_player_loot**](PluginLolLootApi.md#get_lol_loot_v1_player_loot) | **Get** /lol-loot/v1/player-loot | 
[**get_lol_loot_v1_player_loot_by_loot_id**](PluginLolLootApi.md#get_lol_loot_v1_player_loot_by_loot_id) | **Get** /lol-loot/v1/player-loot/{lootId} | 
[**get_lol_loot_v1_player_loot_by_loot_id_context_menu**](PluginLolLootApi.md#get_lol_loot_v1_player_loot_by_loot_id_context_menu) | **Get** /lol-loot/v1/player-loot/{lootId}/context-menu | 
[**get_lol_loot_v1_player_loot_map**](PluginLolLootApi.md#get_lol_loot_v1_player_loot_map) | **Get** /lol-loot/v1/player-loot-map | 
[**get_lol_loot_v1_player_loot_notifications**](PluginLolLootApi.md#get_lol_loot_v1_player_loot_notifications) | **Get** /lol-loot/v1/player-loot-notifications | 
[**get_lol_loot_v1_ready**](PluginLolLootApi.md#get_lol_loot_v1_ready) | **Get** /lol-loot/v1/ready | 
[**get_lol_loot_v1_recipes_configuration**](PluginLolLootApi.md#get_lol_loot_v1_recipes_configuration) | **Get** /lol-loot/v1/recipes/configuration | 
[**get_lol_loot_v1_recipes_initial_item_by_loot_id**](PluginLolLootApi.md#get_lol_loot_v1_recipes_initial_item_by_loot_id) | **Get** /lol-loot/v1/recipes/initial-item/{lootId} | 
[**get_lol_loot_v2_player_loot_map**](PluginLolLootApi.md#get_lol_loot_v2_player_loot_map) | **Get** /lol-loot/v2/player-loot-map | 
[**post_lol_loot_v1_new_player_check_done_by_new_value**](PluginLolLootApi.md#post_lol_loot_v1_new_player_check_done_by_new_value) | **Post** /lol-loot/v1/new-player-check-done/{newValue} | 
[**post_lol_loot_v1_player_loot_by_loot_id_context_menu**](PluginLolLootApi.md#post_lol_loot_v1_player_loot_by_loot_id_context_menu) | **Post** /lol-loot/v1/player-loot/{lootId}/context-menu | 
[**post_lol_loot_v1_player_loot_by_loot_name_redeem**](PluginLolLootApi.md#post_lol_loot_v1_player_loot_by_loot_name_redeem) | **Post** /lol-loot/v1/player-loot/{lootName}/redeem | 
[**post_lol_loot_v1_player_loot_notifications_by_id_acknowledge**](PluginLolLootApi.md#post_lol_loot_v1_player_loot_notifications_by_id_acknowledge) | **Post** /lol-loot/v1/player-loot-notifications/{id}/acknowledge | 
[**post_lol_loot_v1_recipes_by_recipe_name_craft**](PluginLolLootApi.md#post_lol_loot_v1_recipes_by_recipe_name_craft) | **Post** /lol-loot/v1/recipes/{recipeName}/craft | 
[**post_lol_loot_v1_recipes_initial_item_by_loot_id**](PluginLolLootApi.md#post_lol_loot_v1_recipes_initial_item_by_loot_id) | **Post** /lol-loot/v1/recipes/initial-item/{lootId} | 
[**post_lol_loot_v1_refresh**](PluginLolLootApi.md#post_lol_loot_v1_refresh) | **Post** /lol-loot/v1/refresh | 



## delete_lol_loot_v1_loot_grants_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_loot_v1_loot_grants_by_id(id)


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


## delete_lol_loot_v1_player_loot_by_loot_id_new_notification

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_loot_v1_player_loot_by_loot_id_new_notification(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_currency_configuration

> get_lol_loot_v1_currency_configuration()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_enabled

> bool get_lol_loot_v1_enabled()


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


## get_lol_loot_v1_loot_grants

> Vec<crate::models::LolLootLootGrantNotification> get_lol_loot_v1_loot_grants()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLootLootGrantNotification>**](LolLootLootGrantNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_loot_items

> Vec<crate::models::LolLootLootItem> get_lol_loot_v1_loot_items()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLootLootItem>**](LolLootLootItem.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_new_player_check_done

> bool get_lol_loot_v1_new_player_check_done()


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


## get_lol_loot_v1_player_display_categories

> Vec<String> get_lol_loot_v1_player_display_categories()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_player_loot

> Vec<crate::models::LolLootPlayerLoot> get_lol_loot_v1_player_loot()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLootPlayerLoot>**](LolLootPlayerLoot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_player_loot_by_loot_id

> crate::models::LolLootPlayerLoot get_lol_loot_v1_player_loot_by_loot_id(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**crate::models::LolLootPlayerLoot**](LolLootPlayerLoot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_player_loot_by_loot_id_context_menu

> Vec<crate::models::LolLootContextMenu> get_lol_loot_v1_player_loot_by_loot_id_context_menu(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolLootContextMenu>**](LolLootContextMenu.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_player_loot_map

> ::std::collections::HashMap<String, crate::models::LolLootPlayerLoot> get_lol_loot_v1_player_loot_map()


### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, crate::models::LolLootPlayerLoot>**](LolLootPlayerLoot.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_player_loot_notifications

> Vec<crate::models::LolLootPlayerLootNotification> get_lol_loot_v1_player_loot_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolLootPlayerLootNotification>**](LolLootPlayerLootNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_ready

> bool get_lol_loot_v1_ready()


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


## get_lol_loot_v1_recipes_configuration

> get_lol_loot_v1_recipes_configuration()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v1_recipes_initial_item_by_loot_id

> Vec<crate::models::LolLootRecipe> get_lol_loot_v1_recipes_initial_item_by_loot_id(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolLootRecipe>**](LolLootRecipe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_loot_v2_player_loot_map

> crate::models::LolLootPlayerLootMap get_lol_loot_v2_player_loot_map()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLootPlayerLootMap**](LolLootPlayerLootMap.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_new_player_check_done_by_new_value

> String post_lol_loot_v1_new_player_check_done_by_new_value(new_value)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_value** | **bool** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_player_loot_by_loot_id_context_menu

> Vec<crate::models::LolLootContextMenu> post_lol_loot_v1_player_loot_by_loot_id_context_menu(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolLootContextMenu>**](LolLootContextMenu.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_player_loot_by_loot_name_redeem

> crate::models::LolLootPlayerLootUpdate post_lol_loot_v1_player_loot_by_loot_name_redeem(loot_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_name** | **String** |  | [required] |

### Return type

[**crate::models::LolLootPlayerLootUpdate**](LolLootPlayerLootUpdate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_player_loot_notifications_by_id_acknowledge

> String post_lol_loot_v1_player_loot_notifications_by_id_acknowledge(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_recipes_by_recipe_name_craft

> crate::models::LolLootPlayerLootUpdate post_lol_loot_v1_recipes_by_recipe_name_craft(recipe_name, player_loot_list, repeat)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**recipe_name** | **String** |  | [required] |
**player_loot_list** | [**Vec<String>**](String.md) |  | [required] |
**repeat** | Option<**i32**> |  |  |

### Return type

[**crate::models::LolLootPlayerLootUpdate**](LolLootPlayerLootUpdate.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_recipes_initial_item_by_loot_id

> Vec<crate::models::LolLootRecipe> post_lol_loot_v1_recipes_initial_item_by_loot_id(loot_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**loot_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolLootRecipe>**](LolLootRecipe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_loot_v1_refresh

> String post_lol_loot_v1_refresh(force)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force** | **bool** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

