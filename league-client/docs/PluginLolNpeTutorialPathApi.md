# \PluginLolNpeTutorialPathApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_lol_npe_tutorial_path_v1_rewards_champ**](PluginLolNpeTutorialPathApi.md#get_lol_npe_tutorial_path_v1_rewards_champ) | **Get** /lol-npe-tutorial-path/v1/rewards/champ | 
[**get_lol_npe_tutorial_path_v1_settings**](PluginLolNpeTutorialPathApi.md#get_lol_npe_tutorial_path_v1_settings) | **Get** /lol-npe-tutorial-path/v1/settings | 
[**get_lol_npe_tutorial_path_v1_tutorials**](PluginLolNpeTutorialPathApi.md#get_lol_npe_tutorial_path_v1_tutorials) | **Get** /lol-npe-tutorial-path/v1/tutorials | 
[**patch_lol_npe_tutorial_path_v1_tutorials_init**](PluginLolNpeTutorialPathApi.md#patch_lol_npe_tutorial_path_v1_tutorials_init) | **Patch** /lol-npe-tutorial-path/v1/tutorials/init | 
[**put_lol_npe_tutorial_path_v1_settings**](PluginLolNpeTutorialPathApi.md#put_lol_npe_tutorial_path_v1_settings) | **Put** /lol-npe-tutorial-path/v1/settings | 
[**put_lol_npe_tutorial_path_v1_tutorials_by_tutorial_id_view**](PluginLolNpeTutorialPathApi.md#put_lol_npe_tutorial_path_v1_tutorials_by_tutorial_id_view) | **Put** /lol-npe-tutorial-path/v1/tutorials/{tutorialId}/view | 



## get_lol_npe_tutorial_path_v1_rewards_champ

> crate::models::LolNpeTutorialPathCollectionsChampion get_lol_npe_tutorial_path_v1_rewards_champ()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolNpeTutorialPathCollectionsChampion**](LolNpeTutorialPathCollectionsChampion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_npe_tutorial_path_v1_settings

> crate::models::LolNpeTutorialPathAccountSettingsTutorial get_lol_npe_tutorial_path_v1_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolNpeTutorialPathAccountSettingsTutorial**](LolNpeTutorialPathAccountSettingsTutorial.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_npe_tutorial_path_v1_tutorials

> Vec<crate::models::LolNpeTutorialPathTutorial> get_lol_npe_tutorial_path_v1_tutorials()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolNpeTutorialPathTutorial>**](LolNpeTutorialPathTutorial.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_npe_tutorial_path_v1_tutorials_init

> patch_lol_npe_tutorial_path_v1_tutorials_init()


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


## put_lol_npe_tutorial_path_v1_settings

> put_lol_npe_tutorial_path_v1_settings(settings)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | [**LolNpeTutorialPathAccountSettingsTutorial**](LolNpeTutorialPathAccountSettingsTutorial.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_npe_tutorial_path_v1_tutorials_by_tutorial_id_view

> put_lol_npe_tutorial_path_v1_tutorials_by_tutorial_id_view(tutorial_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tutorial_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

