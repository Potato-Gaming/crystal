# \PluginRecofrienderApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_recofriender_v1_registrations_by_network**](PluginRecofrienderApi.md#delete_recofriender_v1_registrations_by_network) | **Delete** /recofriender/v1/registrations/{network} | 
[**delete_recofriender_v2_contacts**](PluginRecofrienderApi.md#delete_recofriender_v2_contacts) | **Delete** /recofriender/v2/contacts | 
[**delete_recofriender_v2_dismissed**](PluginRecofrienderApi.md#delete_recofriender_v2_dismissed) | **Delete** /recofriender/v2/dismissed | 
[**get_recofriender_v1_config**](PluginRecofrienderApi.md#get_recofriender_v1_config) | **Get** /recofriender/v1/config | 
[**get_recofriender_v1_config_by_network**](PluginRecofrienderApi.md#get_recofriender_v1_config_by_network) | **Get** /recofriender/v1/config/{network} | 
[**get_recofriender_v1_contacts**](PluginRecofrienderApi.md#get_recofriender_v1_contacts) | **Get** /recofriender/v1/contacts | 
[**get_recofriender_v1_debug**](PluginRecofrienderApi.md#get_recofriender_v1_debug) | **Get** /recofriender/v1/debug | 
[**get_recofriender_v1_faq_url**](PluginRecofrienderApi.md#get_recofriender_v1_faq_url) | **Get** /recofriender/v1/faq-url | 
[**get_recofriender_v1_registrations**](PluginRecofrienderApi.md#get_recofriender_v1_registrations) | **Get** /recofriender/v1/registrations | 
[**get_recofriender_v1_registrations_by_network**](PluginRecofrienderApi.md#get_recofriender_v1_registrations_by_network) | **Get** /recofriender/v1/registrations/{network} | 
[**get_recofriender_v2_contacts**](PluginRecofrienderApi.md#get_recofriender_v2_contacts) | **Get** /recofriender/v2/contacts | 
[**get_recofriender_v2_contacts_by_account_id**](PluginRecofrienderApi.md#get_recofriender_v2_contacts_by_account_id) | **Get** /recofriender/v2/contacts/{accountId} | 
[**get_recofriender_v2_contacts_page**](PluginRecofrienderApi.md#get_recofriender_v2_contacts_page) | **Get** /recofriender/v2/contacts/page | 
[**get_recofriender_v2_dismissed**](PluginRecofrienderApi.md#get_recofriender_v2_dismissed) | **Get** /recofriender/v2/dismissed | 
[**get_recofriender_v2_dismissed_page**](PluginRecofrienderApi.md#get_recofriender_v2_dismissed_page) | **Get** /recofriender/v2/dismissed/page | 
[**post_recofriender_v1_contacts_by_account_id_available**](PluginRecofrienderApi.md#post_recofriender_v1_contacts_by_account_id_available) | **Post** /recofriender/v1/contacts/{accountId}/available | 
[**post_recofriender_v1_contacts_by_account_id_dismissed**](PluginRecofrienderApi.md#post_recofriender_v1_contacts_by_account_id_dismissed) | **Post** /recofriender/v1/contacts/{accountId}/dismissed | 
[**post_recofriender_v1_contacts_by_account_id_invited**](PluginRecofrienderApi.md#post_recofriender_v1_contacts_by_account_id_invited) | **Post** /recofriender/v1/contacts/{accountId}/invited | 
[**post_recofriender_v1_registrations_by_network**](PluginRecofrienderApi.md#post_recofriender_v1_registrations_by_network) | **Post** /recofriender/v1/registrations/{network} | 
[**put_recofriender_v1_debug**](PluginRecofrienderApi.md#put_recofriender_v1_debug) | **Put** /recofriender/v1/debug | 



## delete_recofriender_v1_registrations_by_network

> ::std::collections::HashMap<String, serde_json::Value> delete_recofriender_v1_registrations_by_network(network)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recofriender_v2_contacts

> delete_recofriender_v2_contacts()


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


## delete_recofriender_v2_dismissed

> delete_recofriender_v2_dismissed()


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


## get_recofriender_v1_config

> crate::models::RecofrienderConfig get_recofriender_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RecofrienderConfig**](RecofrienderConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_config_by_network

> crate::models::RecofrienderNetworkConfig get_recofriender_v1_config_by_network(network)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** |  | [required] |

### Return type

[**crate::models::RecofrienderNetworkConfig**](RecofrienderNetworkConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_contacts

> Vec<crate::models::RecofrienderContactResource> get_recofriender_v1_contacts(account_id, source, friend_state)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**i64**> |  |  |
**source** | Option<**String**> |  |  |
**friend_state** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RecofrienderContactResource>**](RecofrienderContactResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_debug

> crate::models::RecofrienderDebugConfig get_recofriender_v1_debug()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RecofrienderDebugConfig**](RecofrienderDebugConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_faq_url

> crate::models::RecofrienderUrlResource get_recofriender_v1_faq_url()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RecofrienderUrlResource**](RecofrienderUrlResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_registrations

> Vec<crate::models::RecofrienderLinkResource> get_recofriender_v1_registrations(cb)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cb** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::RecofrienderLinkResource>**](RecofrienderLinkResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v1_registrations_by_network

> crate::models::RecofrienderLinkResource get_recofriender_v1_registrations_by_network(network)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** |  | [required] |

### Return type

[**crate::models::RecofrienderLinkResource**](RecofrienderLinkResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v2_contacts

> Vec<crate::models::RecofrienderContactResource> get_recofriender_v2_contacts()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RecofrienderContactResource>**](RecofrienderContactResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v2_contacts_by_account_id

> crate::models::RecofrienderContactResource get_recofriender_v2_contacts_by_account_id(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |

### Return type

[**crate::models::RecofrienderContactResource**](RecofrienderContactResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v2_contacts_page

> crate::models::RecofrienderContactPaginationResource get_recofriender_v2_contacts_page(start, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **i64** |  | [required] |
**limit** | **i64** |  | [required] |

### Return type

[**crate::models::RecofrienderContactPaginationResource**](RecofrienderContactPaginationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v2_dismissed

> Vec<crate::models::RecofrienderContactResource> get_recofriender_v2_dismissed()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::RecofrienderContactResource>**](RecofrienderContactResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recofriender_v2_dismissed_page

> crate::models::RecofrienderContactPaginationResource get_recofriender_v2_dismissed_page(start, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start** | **i64** |  | [required] |
**limit** | **i64** |  | [required] |

### Return type

[**crate::models::RecofrienderContactPaginationResource**](RecofrienderContactPaginationResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recofriender_v1_contacts_by_account_id_available

> crate::models::RecofrienderContactStateResource post_recofriender_v1_contacts_by_account_id_available(account_id, retain_in_cache)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**retain_in_cache** | Option<**bool**> |  |  |

### Return type

[**crate::models::RecofrienderContactStateResource**](RecofrienderContactStateResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recofriender_v1_contacts_by_account_id_dismissed

> crate::models::RecofrienderContactStateResource post_recofriender_v1_contacts_by_account_id_dismissed(account_id, retain_in_cache)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**retain_in_cache** | Option<**bool**> |  |  |

### Return type

[**crate::models::RecofrienderContactStateResource**](RecofrienderContactStateResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recofriender_v1_contacts_by_account_id_invited

> crate::models::RecofrienderContactStateResource post_recofriender_v1_contacts_by_account_id_invited(account_id, retain_in_cache)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i64** |  | [required] |
**retain_in_cache** | Option<**bool**> |  |  |

### Return type

[**crate::models::RecofrienderContactStateResource**](RecofrienderContactStateResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_recofriender_v1_registrations_by_network

> crate::models::RecofrienderUrlResource post_recofriender_v1_registrations_by_network(network)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** |  | [required] |

### Return type

[**crate::models::RecofrienderUrlResource**](RecofrienderUrlResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_recofriender_v1_debug

> crate::models::RecofrienderDebugConfig put_recofriender_v1_debug(debug_configuration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**debug_configuration** | [**RecofrienderDebugConfig**](RecofrienderDebugConfig.md) |  | [required] |

### Return type

[**crate::models::RecofrienderDebugConfig**](RecofrienderDebugConfig.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

