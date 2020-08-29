# \BuiltinApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**async_delete**](BuiltinApi.md#async_delete) | **Post** /AsyncDelete | Cancels the asynchronous operation or removes its completion status.
[**async_result**](BuiltinApi.md#async_result) | **Post** /AsyncResult | Retrieves the result of a completed asynchronous operation.
[**async_status**](BuiltinApi.md#async_status) | **Post** /AsyncStatus | Retrieves details on the current state of an asynchronous operation.
[**cancel**](BuiltinApi.md#cancel) | **Post** /Cancel | Attempts to cancel an asynchronous operation
[**exit**](BuiltinApi.md#exit) | **Post** /Exit | Closes the connection.
[**help**](BuiltinApi.md#help) | **Post** /Help | Returns information on available functions and types
[**http2_api_docs_v2**](BuiltinApi.md#http2_api_docs_v2) | **Get** /swagger/v2/swagger.json | Retrieves the API documentation
[**http_api_declaration_v1**](BuiltinApi.md#http_api_declaration_v1) | **Get** /swagger/v1/api-docs/{api} | Retrieves the API declaration for a supported API
[**http_api_docs_v1**](BuiltinApi.md#http_api_docs_v1) | **Get** /swagger/v1/api-docs | Retrieves the API documentation resource listing
[**http_api_docs_v3**](BuiltinApi.md#http_api_docs_v3) | **Get** /swagger/v3/openapi.json | Retrieves the API documentation
[**http_async_delete**](BuiltinApi.md#http_async_delete) | **Delete** /async/v1/status/{asyncToken} | Cancels the asynchronous operation or removes its completion status.
[**http_async_result**](BuiltinApi.md#http_async_result) | **Get** /async/v1/result/{asyncToken} | Retrieves the result of a completed asynchronous operation.
[**http_async_status**](BuiltinApi.md#http_async_status) | **Get** /async/v1/status/{asyncToken} | Retrieves details on the current state of an asynchronous operation.
[**subscribe**](BuiltinApi.md#subscribe) | **Post** /Subscribe | Subscribes to a given event
[**unsubscribe**](BuiltinApi.md#unsubscribe) | **Post** /Unsubscribe | Unsubscribes from a given event
[**web_socket_format**](BuiltinApi.md#web_socket_format) | **Post** /WebSocketFormat | Controls the console output format



## async_delete

> ::std::collections::HashMap<String, serde_json::Value> async_delete(async_token)
Cancels the asynchronous operation or removes its completion status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to remove | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## async_result

> ::std::collections::HashMap<String, serde_json::Value> async_result(async_token)
Retrieves the result of a completed asynchronous operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to check | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## async_status

> ::std::collections::HashMap<String, serde_json::Value> async_status(async_token)
Retrieves details on the current state of an asynchronous operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to check | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel

> ::std::collections::HashMap<String, serde_json::Value> cancel(async_token)
Attempts to cancel an asynchronous operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | Operation to cancel | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exit

> ::std::collections::HashMap<String, serde_json::Value> exit()
Closes the connection.

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


## help

> ::std::collections::HashMap<String, serde_json::Value> help(target, format)
Returns information on available functions and types

With no arguments, returns a list of all available functions and types along with a short description. If a function or type is specified, returns detailed information about it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**target** | Option<**String**> | Name of the function or type to describe |  |
**format** | Option<**String**> | Format for returned information |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## http2_api_docs_v2

> ::std::collections::HashMap<String, serde_json::Value> http2_api_docs_v2()
Retrieves the API documentation

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


## http_api_declaration_v1

> ::std::collections::HashMap<String, serde_json::Value> http_api_declaration_v1(api, api2)
Retrieves the API declaration for a supported API

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api** | **String** | API to get a declaration for | [required] |
**api2** | **String** | API to get a declaration for | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## http_api_docs_v1

> ::std::collections::HashMap<String, serde_json::Value> http_api_docs_v1()
Retrieves the API documentation resource listing

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


## http_api_docs_v3

> ::std::collections::HashMap<String, serde_json::Value> http_api_docs_v3()
Retrieves the API documentation

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


## http_async_delete

> ::std::collections::HashMap<String, serde_json::Value> http_async_delete(async_token, async_token2)
Cancels the asynchronous operation or removes its completion status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to remove | [required] |
**async_token2** | **i32** | ID of the asynchronous operation to remove | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## http_async_result

> ::std::collections::HashMap<String, serde_json::Value> http_async_result(async_token, async_token2)
Retrieves the result of a completed asynchronous operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to check | [required] |
**async_token2** | **i32** | ID of the asynchronous operation to check | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## http_async_status

> ::std::collections::HashMap<String, serde_json::Value> http_async_status(async_token, async_token2)
Retrieves details on the current state of an asynchronous operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**async_token** | **i32** | ID of the asynchronous operation to check | [required] |
**async_token2** | **i32** | ID of the asynchronous operation to check | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe

> ::std::collections::HashMap<String, serde_json::Value> subscribe(event_name, format)
Subscribes to a given event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | Name of the event to subscribe to | [required] |
**format** | Option<**String**> | Desired format to receive events in. If unspecified, events will be sent in the active result format at the time. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unsubscribe

> ::std::collections::HashMap<String, serde_json::Value> unsubscribe(event_name)
Unsubscribes from a given event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_name** | **String** | Name of the event to unsubscribe from | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_socket_format

> ::std::collections::HashMap<String, serde_json::Value> web_socket_format(format)
Controls the console output format

With no arguments, returns the current output format being used. If a format is specified, switches the console output to that format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | Output format to switch to |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

