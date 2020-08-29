# \PluginRiotMessagingServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_riot_messaging_service_v1_connect**](PluginRiotMessagingServiceApi.md#delete_riot_messaging_service_v1_connect) | **Delete** /riot-messaging-service/v1/connect | 
[**delete_riot_messaging_service_v1_entitlements**](PluginRiotMessagingServiceApi.md#delete_riot_messaging_service_v1_entitlements) | **Delete** /riot-messaging-service/v1/entitlements | 
[**delete_riot_messaging_service_v1_session**](PluginRiotMessagingServiceApi.md#delete_riot_messaging_service_v1_session) | **Delete** /riot-messaging-service/v1/session | 
[**get_riot_messaging_service_v1_message_by_a**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a) | **Get** /riot-messaging-service/v1/message/{a} | 
[**get_riot_messaging_service_v1_message_by_a_by_b**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a_by_b) | **Get** /riot-messaging-service/v1/message/{a}/{b} | 
[**get_riot_messaging_service_v1_message_by_a_by_b_by_c**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a_by_b_by_c) | **Get** /riot-messaging-service/v1/message/{a}/{b}/{c} | 
[**get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d) | **Get** /riot-messaging-service/v1/message/{a}/{b}/{c}/{d} | 
[**get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e) | **Get** /riot-messaging-service/v1/message/{a}/{b}/{c}/{d}/{e} | 
[**get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e_by_f**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e_by_f) | **Get** /riot-messaging-service/v1/message/{a}/{b}/{c}/{d}/{e}/{f} | 
[**get_riot_messaging_service_v1_session**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_session) | **Get** /riot-messaging-service/v1/session | 
[**get_riot_messaging_service_v1_state**](PluginRiotMessagingServiceApi.md#get_riot_messaging_service_v1_state) | **Get** /riot-messaging-service/v1/state | 
[**post_riot_messaging_service_v1_connect**](PluginRiotMessagingServiceApi.md#post_riot_messaging_service_v1_connect) | **Post** /riot-messaging-service/v1/connect | 
[**post_riot_messaging_service_v1_entitlements**](PluginRiotMessagingServiceApi.md#post_riot_messaging_service_v1_entitlements) | **Post** /riot-messaging-service/v1/entitlements | 



## delete_riot_messaging_service_v1_connect

> delete_riot_messaging_service_v1_connect()


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


## delete_riot_messaging_service_v1_entitlements

> delete_riot_messaging_service_v1_entitlements()


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


## delete_riot_messaging_service_v1_session

> delete_riot_messaging_service_v1_session()


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


## get_riot_messaging_service_v1_message_by_a

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a(a)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_message_by_a_by_b

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a_by_b(a, b)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |
**b** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_message_by_a_by_b_by_c

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a_by_b_by_c(a, b, c)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |
**b** | **String** |  | [required] |
**c** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d(a, b, c, d)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |
**b** | **String** |  | [required] |
**c** | **String** |  | [required] |
**d** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e(a, b, c, d, e)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |
**b** | **String** |  | [required] |
**c** | **String** |  | [required] |
**d** | **String** |  | [required] |
**e** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e_by_f

> crate::models::RmsMessage get_riot_messaging_service_v1_message_by_a_by_b_by_c_by_d_by_e_by_f(a, b, c, d, e, f)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**a** | **String** |  | [required] |
**b** | **String** |  | [required] |
**c** | **String** |  | [required] |
**d** | **String** |  | [required] |
**e** | **String** |  | [required] |
**f** | **String** |  | [required] |

### Return type

[**crate::models::RmsMessage**](RmsMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_session

> crate::models::RiotMessagingServiceSession get_riot_messaging_service_v1_session()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RiotMessagingServiceSession**](RiotMessagingServiceSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_riot_messaging_service_v1_state

> crate::models::RiotMessagingServiceState get_riot_messaging_service_v1_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RiotMessagingServiceState**](RiotMessagingServiceState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riot_messaging_service_v1_connect

> post_riot_messaging_service_v1_connect(id_token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id_token** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_riot_messaging_service_v1_entitlements

> post_riot_messaging_service_v1_entitlements(token)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | [**RiotMessagingServiceEntitlementsToken**](RiotMessagingServiceEntitlementsToken.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

