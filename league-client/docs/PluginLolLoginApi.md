# \PluginLolLoginApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name**](PluginLolLoginApi.md#delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name) | **Delete** /lol-login/v1/service-proxy-async-requests/{serviceName}/{methodName} | 
[**delete_lol_login_v1_session**](PluginLolLoginApi.md#delete_lol_login_v1_session) | **Delete** /lol-login/v1/session | 
[**delete_lol_login_v1_shutdown_locks_by_lock_name**](PluginLolLoginApi.md#delete_lol_login_v1_shutdown_locks_by_lock_name) | **Delete** /lol-login/v1/shutdown-locks/{lockName} | 
[**get_lol_login_v1_account_state**](PluginLolLoginApi.md#get_lol_login_v1_account_state) | **Get** /lol-login/v1/account-state | 
[**get_lol_login_v1_login_connection_state**](PluginLolLoginApi.md#get_lol_login_v1_login_connection_state) | **Get** /lol-login/v1/login-connection-state | 
[**get_lol_login_v1_login_data_packet**](PluginLolLoginApi.md#get_lol_login_v1_login_data_packet) | **Get** /lol-login/v1/login-data-packet | 
[**get_lol_login_v1_login_in_game_creds**](PluginLolLoginApi.md#get_lol_login_v1_login_in_game_creds) | **Get** /lol-login/v1/login-in-game-creds | 
[**get_lol_login_v1_login_platform_credentials**](PluginLolLoginApi.md#get_lol_login_v1_login_platform_credentials) | **Get** /lol-login/v1/login-platform-credentials | 
[**get_lol_login_v1_login_queue_state**](PluginLolLoginApi.md#get_lol_login_v1_login_queue_state) | **Get** /lol-login/v1/login-queue-state | 
[**get_lol_login_v1_session**](PluginLolLoginApi.md#get_lol_login_v1_session) | **Get** /lol-login/v1/session | 
[**get_lol_login_v1_wallet**](PluginLolLoginApi.md#get_lol_login_v1_wallet) | **Get** /lol-login/v1/wallet | 
[**get_lol_login_v2_league_session_init_token**](PluginLolLoginApi.md#get_lol_login_v2_league_session_init_token) | **Get** /lol-login/v2/league-session-init-token | 
[**post_lol_login_v1_account_state**](PluginLolLoginApi.md#post_lol_login_v1_account_state) | **Post** /lol-login/v1/account-state | 
[**post_lol_login_v1_change_summoner_name**](PluginLolLoginApi.md#post_lol_login_v1_change_summoner_name) | **Post** /lol-login/v1/change-summoner-name | 
[**post_lol_login_v1_delete_rso_on_close**](PluginLolLoginApi.md#post_lol_login_v1_delete_rso_on_close) | **Post** /lol-login/v1/delete-rso-on-close | 
[**post_lol_login_v1_league_session_status**](PluginLolLoginApi.md#post_lol_login_v1_league_session_status) | **Post** /lol-login/v1/leagueSessionStatus | 
[**post_lol_login_v1_new_player_flow_completed**](PluginLolLoginApi.md#post_lol_login_v1_new_player_flow_completed) | **Post** /lol-login/v1/new-player-flow-completed | 
[**post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name**](PluginLolLoginApi.md#post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name) | **Post** /lol-login/v1/service-proxy-async-requests/{serviceName}/{methodName} | 
[**post_lol_login_v1_service_proxy_uuid_requests**](PluginLolLoginApi.md#post_lol_login_v1_service_proxy_uuid_requests) | **Post** /lol-login/v1/service-proxy-uuid-requests | 
[**post_lol_login_v1_session**](PluginLolLoginApi.md#post_lol_login_v1_session) | **Post** /lol-login/v1/session | 
[**post_lol_login_v1_session_invoke**](PluginLolLoginApi.md#post_lol_login_v1_session_invoke) | **Post** /lol-login/v1/session/invoke | 
[**post_lol_login_v1_summoner_created**](PluginLolLoginApi.md#post_lol_login_v1_summoner_created) | **Post** /lol-login/v1/summoner-created | 
[**put_lol_login_v1_shutdown_locks_by_lock_name**](PluginLolLoginApi.md#put_lol_login_v1_shutdown_locks_by_lock_name) | **Put** /lol-login/v1/shutdown-locks/{lockName} | 



## delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name

> delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(service_name, method_name, plugin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**method_name** | **String** |  | [required] |
**plugin_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_login_v1_session

> delete_lol_login_v1_session()


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


## delete_lol_login_v1_shutdown_locks_by_lock_name

> delete_lol_login_v1_shutdown_locks_by_lock_name(lock_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lock_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_account_state

> crate::models::LolLoginAccountStateResource get_lol_login_v1_account_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginAccountStateResource**](LolLoginAccountStateResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_login_connection_state

> crate::models::LolLoginLoginConnectionState get_lol_login_v1_login_connection_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginLoginConnectionState**](LolLoginLoginConnectionState.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_login_data_packet

> ::std::collections::HashMap<String, serde_json::Value> get_lol_login_v1_login_data_packet()


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


## get_lol_login_v1_login_in_game_creds

> ::std::collections::HashMap<String, serde_json::Value> get_lol_login_v1_login_in_game_creds()


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


## get_lol_login_v1_login_platform_credentials

> crate::models::LolLoginPlatformGeneratedCredentials get_lol_login_v1_login_platform_credentials()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginPlatformGeneratedCredentials**](LolLoginPlatformGeneratedCredentials.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_login_queue_state

> crate::models::LolLoginLoginQueue get_lol_login_v1_login_queue_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginLoginQueue**](LolLoginLoginQueue.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_session

> crate::models::LolLoginLoginSession get_lol_login_v1_session()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginLoginSession**](LolLoginLoginSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v1_wallet

> crate::models::LolLoginLoginSessionWallet get_lol_login_v1_wallet()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginLoginSessionWallet**](LolLoginLoginSessionWallet.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_login_v2_league_session_init_token

> crate::models::LolLoginLeagueSessionTokenEnvelope get_lol_login_v2_league_session_init_token()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolLoginLeagueSessionTokenEnvelope**](LolLoginLeagueSessionTokenEnvelope.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_account_state

> post_lol_login_v1_account_state()


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


## post_lol_login_v1_change_summoner_name

> ::std::collections::HashMap<String, serde_json::Value> post_lol_login_v1_change_summoner_name(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_delete_rso_on_close

> ::std::collections::HashMap<String, serde_json::Value> post_lol_login_v1_delete_rso_on_close()


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


## post_lol_login_v1_league_session_status

> post_lol_login_v1_league_session_status(league_session)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**league_session** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_new_player_flow_completed

> ::std::collections::HashMap<String, serde_json::Value> post_lol_login_v1_new_player_flow_completed()


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


## post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name

> post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(service_name, method_name, plugin_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**method_name** | **String** |  | [required] |
**plugin_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_service_proxy_uuid_requests

> String post_lol_login_v1_service_proxy_uuid_requests(service_name, method_name, plugin_id, timeout_millis, payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_name** | **String** |  | [required] |
**method_name** | **String** |  | [required] |
**plugin_id** | **i32** |  | [required] |
**timeout_millis** | **i64** |  | [required] |
**payload** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_session

> crate::models::LolLoginLoginSession post_lol_login_v1_session(username_and_password)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username_and_password** | [**LolLoginUsernameAndPassword**](LolLoginUsernameAndPassword.md) |  | [required] |

### Return type

[**crate::models::LolLoginLoginSession**](LolLoginLoginSession.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_session_invoke

> crate::models::LolLoginLcdsResponse post_lol_login_v1_session_invoke(destination, method, UNKNOWN_BASE_TYPE)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination** | **String** |  | [required] |
**method** | **String** |  | [required] |
**UNKNOWN_BASE_TYPE** | [**UNKNOWN_BASE_TYPE**](UNKNOWN_BASE_TYPE.md) |  | [required] |

### Return type

[**crate::models::LolLoginLcdsResponse**](LolLoginLcdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_login_v1_summoner_created

> ::std::collections::HashMap<String, serde_json::Value> post_lol_login_v1_summoner_created(summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**summoner_id** | [**LolLoginSummonerCreatedResource**](LolLoginSummonerCreatedResource.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_login_v1_shutdown_locks_by_lock_name

> put_lol_login_v1_shutdown_locks_by_lock_name(lock_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lock_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

