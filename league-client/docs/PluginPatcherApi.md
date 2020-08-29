# \PluginPatcherApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_patcher_v1_notifications_by_id**](PluginPatcherApi.md#delete_patcher_v1_notifications_by_id) | **Delete** /patcher/v1/notifications/{id} | 
[**delete_patcher_v1_products_by_product_id**](PluginPatcherApi.md#delete_patcher_v1_products_by_product_id) | **Delete** /patcher/v1/products/{product-id} | 
[**get_patcher_v1_notifications**](PluginPatcherApi.md#get_patcher_v1_notifications) | **Get** /patcher/v1/notifications | 
[**get_patcher_v1_p2p_status**](PluginPatcherApi.md#get_patcher_v1_p2p_status) | **Get** /patcher/v1/p2p/status | 
[**get_patcher_v1_products**](PluginPatcherApi.md#get_patcher_v1_products) | **Get** /patcher/v1/products | 
[**get_patcher_v1_products_by_product_id_paths**](PluginPatcherApi.md#get_patcher_v1_products_by_product_id_paths) | **Get** /patcher/v1/products/{product-id}/paths | 
[**get_patcher_v1_products_by_product_id_state**](PluginPatcherApi.md#get_patcher_v1_products_by_product_id_state) | **Get** /patcher/v1/products/{product-id}/state | 
[**get_patcher_v1_products_by_product_id_tags**](PluginPatcherApi.md#get_patcher_v1_products_by_product_id_tags) | **Get** /patcher/v1/products/{product-id}/tags | 
[**get_patcher_v1_status**](PluginPatcherApi.md#get_patcher_v1_status) | **Get** /patcher/v1/status | 
[**patch_patcher_v1_p2p_status**](PluginPatcherApi.md#patch_patcher_v1_p2p_status) | **Patch** /patcher/v1/p2p/status | 
[**post_patcher_v1_notifications**](PluginPatcherApi.md#post_patcher_v1_notifications) | **Post** /patcher/v1/notifications | 
[**post_patcher_v1_products_by_product_id_detect_corruption_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_detect_corruption_request) | **Post** /patcher/v1/products/{product-id}/detect-corruption-request | 
[**post_patcher_v1_products_by_product_id_partial_repair_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_partial_repair_request) | **Post** /patcher/v1/products/{product-id}/partial-repair-request | 
[**post_patcher_v1_products_by_product_id_signal_start_patching_delayed**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_signal_start_patching_delayed) | **Post** /patcher/v1/products/{product-id}/signal-start-patching-delayed | 
[**post_patcher_v1_products_by_product_id_start_checking_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_start_checking_request) | **Post** /patcher/v1/products/{product-id}/start-checking-request | 
[**post_patcher_v1_products_by_product_id_start_patching_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_start_patching_request) | **Post** /patcher/v1/products/{product-id}/start-patching-request | 
[**post_patcher_v1_products_by_product_id_stop_checking_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_stop_checking_request) | **Post** /patcher/v1/products/{product-id}/stop-checking-request | 
[**post_patcher_v1_products_by_product_id_stop_patching_request**](PluginPatcherApi.md#post_patcher_v1_products_by_product_id_stop_patching_request) | **Post** /patcher/v1/products/{product-id}/stop-patching-request | 
[**post_patcher_v1_products_league_of_legends_full_repair_request**](PluginPatcherApi.md#post_patcher_v1_products_league_of_legends_full_repair_request) | **Post** /patcher/v1/products/league_of_legends/full-repair-request | 
[**put_patcher_v1_self_update_restart**](PluginPatcherApi.md#put_patcher_v1_self_update_restart) | **Put** /patcher/v1/self-update-restart | 
[**put_patcher_v1_ux**](PluginPatcherApi.md#put_patcher_v1_ux) | **Put** /patcher/v1/ux | 



## delete_patcher_v1_notifications_by_id

> ::std::collections::HashMap<String, serde_json::Value> delete_patcher_v1_notifications_by_id(id)


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


## delete_patcher_v1_products_by_product_id

> ::std::collections::HashMap<String, serde_json::Value> delete_patcher_v1_products_by_product_id(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_notifications

> Vec<crate::models::PatcherNotification> get_patcher_v1_notifications()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PatcherNotification>**](PatcherNotification.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_p2p_status

> crate::models::PatcherP2PStatus get_patcher_v1_p2p_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PatcherP2PStatus**](PatcherP2PStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_products

> Vec<String> get_patcher_v1_products()


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


## get_patcher_v1_products_by_product_id_paths

> ::std::collections::HashMap<String, String> get_patcher_v1_products_by_product_id_paths(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_products_by_product_id_state

> crate::models::PatcherProductState get_patcher_v1_products_by_product_id_state(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**crate::models::PatcherProductState**](PatcherProductState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_products_by_product_id_tags

> ::std::collections::HashMap<String, String> get_patcher_v1_products_by_product_id_tags(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

**::std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_patcher_v1_status

> crate::models::PatcherStatus get_patcher_v1_status()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::PatcherStatus**](PatcherStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_patcher_v1_p2p_status

> ::std::collections::HashMap<String, serde_json::Value> patch_patcher_v1_p2p_status(data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**data** | [**PatcherP2PStatusUpdate**](PatcherP2PStatusUpdate.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_notifications

> post_patcher_v1_notifications(notification_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**notification_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_detect_corruption_request

> crate::models::PatcherProductState post_patcher_v1_products_by_product_id_detect_corruption_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**crate::models::PatcherProductState**](PatcherProductState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_partial_repair_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_partial_repair_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_signal_start_patching_delayed

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_signal_start_patching_delayed(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_start_checking_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_start_checking_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_start_patching_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_start_patching_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_stop_checking_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_stop_checking_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_by_product_id_stop_patching_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_by_product_id_stop_patching_request(product_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**product_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_patcher_v1_products_league_of_legends_full_repair_request

> ::std::collections::HashMap<String, serde_json::Value> post_patcher_v1_products_league_of_legends_full_repair_request()


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


## put_patcher_v1_self_update_restart

> ::std::collections::HashMap<String, serde_json::Value> put_patcher_v1_self_update_restart(force_restart_on_self_update)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**force_restart_on_self_update** | **bool** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_patcher_v1_ux

> put_patcher_v1_ux(ux)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ux** | [**PatcherUxResource**](PatcherUxResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

