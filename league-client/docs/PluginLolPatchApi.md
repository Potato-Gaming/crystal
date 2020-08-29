# \PluginLolPatchApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_patch_v1_notifications_by_id**](PluginLolPatchApi.md#delete_lol_patch_v1_notifications_by_id) | **Delete** /lol-patch/v1/notifications/{id} | 
[**get_lol_patch_v1_checking_enabled**](PluginLolPatchApi.md#get_lol_patch_v1_checking_enabled) | **Get** /lol-patch/v1/checking-enabled | 
[**get_lol_patch_v1_environment**](PluginLolPatchApi.md#get_lol_patch_v1_environment) | **Get** /lol-patch/v1/environment | 
[**get_lol_patch_v1_game_version**](PluginLolPatchApi.md#get_lol_patch_v1_game_version) | **Get** /lol-patch/v1/game-version | 
[**get_lol_patch_v1_notifications**](PluginLolPatchApi.md#get_lol_patch_v1_notifications) | **Get** /lol-patch/v1/notifications | 
[**get_lol_patch_v1_products_league_of_legends_install_location**](PluginLolPatchApi.md#get_lol_patch_v1_products_league_of_legends_install_location) | **Get** /lol-patch/v1/products/league_of_legends/install-location | 
[**get_lol_patch_v1_products_league_of_legends_state**](PluginLolPatchApi.md#get_lol_patch_v1_products_league_of_legends_state) | **Get** /lol-patch/v1/products/league_of_legends/state | 
[**get_lol_patch_v1_products_league_of_legends_supported_game_releases**](PluginLolPatchApi.md#get_lol_patch_v1_products_league_of_legends_supported_game_releases) | **Get** /lol-patch/v1/products/league_of_legends/supported-game-releases | 
[**get_lol_patch_v1_status**](PluginLolPatchApi.md#get_lol_patch_v1_status) | **Get** /lol-patch/v1/status | 
[**post_lol_patch_v1_products_league_of_legends_detect_corruption_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_detect_corruption_request) | **Post** /lol-patch/v1/products/league_of_legends/detect-corruption-request | 
[**post_lol_patch_v1_products_league_of_legends_partial_repair_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_partial_repair_request) | **Post** /lol-patch/v1/products/league_of_legends/partial-repair-request | 
[**post_lol_patch_v1_products_league_of_legends_start_checking_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_start_checking_request) | **Post** /lol-patch/v1/products/league_of_legends/start-checking-request | 
[**post_lol_patch_v1_products_league_of_legends_start_patching_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_start_patching_request) | **Post** /lol-patch/v1/products/league_of_legends/start-patching-request | 
[**post_lol_patch_v1_products_league_of_legends_stop_checking_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_stop_checking_request) | **Post** /lol-patch/v1/products/league_of_legends/stop-checking-request | 
[**post_lol_patch_v1_products_league_of_legends_stop_patching_request**](PluginLolPatchApi.md#post_lol_patch_v1_products_league_of_legends_stop_patching_request) | **Post** /lol-patch/v1/products/league_of_legends/stop-patching-request | 
[**put_lol_patch_v1_game_patch_url**](PluginLolPatchApi.md#put_lol_patch_v1_game_patch_url) | **Put** /lol-patch/v1/game-patch-url | 
[**put_lol_patch_v1_self_update_restart**](PluginLolPatchApi.md#put_lol_patch_v1_self_update_restart) | **Put** /lol-patch/v1/self-update-restart | 
[**put_lol_patch_v1_ux**](PluginLolPatchApi.md#put_lol_patch_v1_ux) | **Put** /lol-patch/v1/ux | 



## delete_lol_patch_v1_notifications_by_id

> delete_lol_patch_v1_notifications_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_checking_enabled

> bool get_lol_patch_v1_checking_enabled()


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


## get_lol_patch_v1_environment

> crate::models::LolPatchChunkingPatcherEnvironment get_lol_patch_v1_environment()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPatchChunkingPatcherEnvironment**](LolPatchChunkingPatcherEnvironment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_game_version

> String get_lol_patch_v1_game_version()


### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_notifications

> Vec<crate::models::LolPatchNotification> get_lol_patch_v1_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPatchNotification>**](LolPatchNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_products_league_of_legends_install_location

> crate::models::LolPatchInstallPaths get_lol_patch_v1_products_league_of_legends_install_location()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPatchInstallPaths**](LolPatchInstallPaths.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_products_league_of_legends_state

> crate::models::LolPatchProductState get_lol_patch_v1_products_league_of_legends_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPatchProductState**](LolPatchProductState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_products_league_of_legends_supported_game_releases

> crate::models::LolPatchSupportedGameReleases get_lol_patch_v1_products_league_of_legends_supported_game_releases()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPatchSupportedGameReleases**](LolPatchSupportedGameReleases.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_patch_v1_status

> crate::models::LolPatchStatus get_lol_patch_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPatchStatus**](LolPatchStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_patch_v1_products_league_of_legends_detect_corruption_request

> post_lol_patch_v1_products_league_of_legends_detect_corruption_request()


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


## post_lol_patch_v1_products_league_of_legends_partial_repair_request

> post_lol_patch_v1_products_league_of_legends_partial_repair_request()


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


## post_lol_patch_v1_products_league_of_legends_start_checking_request

> post_lol_patch_v1_products_league_of_legends_start_checking_request()


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


## post_lol_patch_v1_products_league_of_legends_start_patching_request

> post_lol_patch_v1_products_league_of_legends_start_patching_request()


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


## post_lol_patch_v1_products_league_of_legends_stop_checking_request

> post_lol_patch_v1_products_league_of_legends_stop_checking_request()


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


## post_lol_patch_v1_products_league_of_legends_stop_patching_request

> post_lol_patch_v1_products_league_of_legends_stop_patching_request(restart)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**restart** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_patch_v1_game_patch_url

> put_lol_patch_v1_game_patch_url(url)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_patch_v1_self_update_restart

> put_lol_patch_v1_self_update_restart(force_restart_on_self_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_restart_on_self_update** | **bool** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_patch_v1_ux

> put_lol_patch_v1_ux(ux)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ux** | [**LolPatchUxResource**](LolPatchUxResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

