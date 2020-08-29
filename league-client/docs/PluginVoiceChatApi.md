# \PluginVoiceChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_voice_chat_v2_sessions**](PluginVoiceChatApi.md#delete_voice_chat_v2_sessions) | **Delete** /voice-chat/v2/sessions | 
[**delete_voice_chat_v2_sessions_by_id**](PluginVoiceChatApi.md#delete_voice_chat_v2_sessions_by_id) | **Delete** /voice-chat/v2/sessions/{id} | 
[**get_voice_chat_v1_audio_properties**](PluginVoiceChatApi.md#get_voice_chat_v1_audio_properties) | **Get** /voice-chat/v1/audio-properties | 
[**get_voice_chat_v1_call_stats_aggregate**](PluginVoiceChatApi.md#get_voice_chat_v1_call_stats_aggregate) | **Get** /voice-chat/v1/call-stats/aggregate | 
[**get_voice_chat_v1_call_stats_by_id**](PluginVoiceChatApi.md#get_voice_chat_v1_call_stats_by_id) | **Get** /voice-chat/v1/call-stats/{id} | 
[**get_voice_chat_v1_codec_settings**](PluginVoiceChatApi.md#get_voice_chat_v1_codec_settings) | **Get** /voice-chat/v1/codec-settings | 
[**get_voice_chat_v1_config**](PluginVoiceChatApi.md#get_voice_chat_v1_config) | **Get** /voice-chat/v1/config | 
[**get_voice_chat_v1_errors**](PluginVoiceChatApi.md#get_voice_chat_v1_errors) | **Get** /voice-chat/v1/errors | 
[**get_voice_chat_v1_push_to_talk**](PluginVoiceChatApi.md#get_voice_chat_v1_push_to_talk) | **Get** /voice-chat/v1/push-to-talk | 
[**get_voice_chat_v2_devices_capture**](PluginVoiceChatApi.md#get_voice_chat_v2_devices_capture) | **Get** /voice-chat/v2/devices/capture | 
[**get_voice_chat_v2_devices_capture_permission**](PluginVoiceChatApi.md#get_voice_chat_v2_devices_capture_permission) | **Get** /voice-chat/v2/devices/capture/permission | 
[**get_voice_chat_v2_devices_render**](PluginVoiceChatApi.md#get_voice_chat_v2_devices_render) | **Get** /voice-chat/v2/devices/render | 
[**get_voice_chat_v2_sessions**](PluginVoiceChatApi.md#get_voice_chat_v2_sessions) | **Get** /voice-chat/v2/sessions | 
[**get_voice_chat_v2_sessions_by_id**](PluginVoiceChatApi.md#get_voice_chat_v2_sessions_by_id) | **Get** /voice-chat/v2/sessions/{id} | 
[**get_voice_chat_v2_sessions_by_session_id_participants_by_participant_id**](PluginVoiceChatApi.md#get_voice_chat_v2_sessions_by_session_id_participants_by_participant_id) | **Get** /voice-chat/v2/sessions/{sessionId}/participants/{participantId} | 
[**get_voice_chat_v2_settings**](PluginVoiceChatApi.md#get_voice_chat_v2_settings) | **Get** /voice-chat/v2/settings | 
[**get_voice_chat_v2_state**](PluginVoiceChatApi.md#get_voice_chat_v2_state) | **Get** /voice-chat/v2/state | 
[**post_voice_chat_v1_push_to_talk_check_available**](PluginVoiceChatApi.md#post_voice_chat_v1_push_to_talk_check_available) | **Post** /voice-chat/v1/push-to-talk/check-available | 
[**post_voice_chat_v1_sessions_by_id**](PluginVoiceChatApi.md#post_voice_chat_v1_sessions_by_id) | **Post** /voice-chat/v1/sessions/{id} | 
[**post_voice_chat_v2_sessions**](PluginVoiceChatApi.md#post_voice_chat_v2_sessions) | **Post** /voice-chat/v2/sessions | 
[**post_voice_chat_v2_sessions_by_id**](PluginVoiceChatApi.md#post_voice_chat_v2_sessions_by_id) | **Post** /voice-chat/v2/sessions/{id} | 
[**put_voice_chat_v1_codec_settings**](PluginVoiceChatApi.md#put_voice_chat_v1_codec_settings) | **Put** /voice-chat/v1/codec-settings | 
[**put_voice_chat_v1_push_to_talk**](PluginVoiceChatApi.md#put_voice_chat_v1_push_to_talk) | **Put** /voice-chat/v1/push-to-talk | 
[**put_voice_chat_v2_devices_capture_prompt_for_permission**](PluginVoiceChatApi.md#put_voice_chat_v2_devices_capture_prompt_for_permission) | **Put** /voice-chat/v2/devices/capture/prompt-for-permission | 
[**put_voice_chat_v2_sessions_by_session_id_participants_by_participant_id**](PluginVoiceChatApi.md#put_voice_chat_v2_sessions_by_session_id_participants_by_participant_id) | **Put** /voice-chat/v2/sessions/{sessionId}/participants/{participantId} | 
[**put_voice_chat_v2_settings**](PluginVoiceChatApi.md#put_voice_chat_v2_settings) | **Put** /voice-chat/v2/settings | 



## delete_voice_chat_v2_sessions

> delete_voice_chat_v2_sessions()


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


## delete_voice_chat_v2_sessions_by_id

> delete_voice_chat_v2_sessions_by_id(id)


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


## get_voice_chat_v1_audio_properties

> crate::models::VoiceChatAudioPropertiesResource get_voice_chat_v1_audio_properties()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatAudioPropertiesResource**](VoiceChatAudioPropertiesResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_call_stats_aggregate

> crate::models::VoiceChatCallStatsResource get_voice_chat_v1_call_stats_aggregate()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatCallStatsResource**](VoiceChatCallStatsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_call_stats_by_id

> Vec<crate::models::VoiceChatCallStatsResource> get_voice_chat_v1_call_stats_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::VoiceChatCallStatsResource>**](VoiceChatCallStatsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_codec_settings

> crate::models::VoiceChatCodecSettingsResource get_voice_chat_v1_codec_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatCodecSettingsResource**](VoiceChatCodecSettingsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_config

> crate::models::VoiceChatConfigResource get_voice_chat_v1_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatConfigResource**](VoiceChatConfigResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_errors

> Vec<crate::models::VoiceChatVoiceErrorCounterResource> get_voice_chat_v1_errors()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VoiceChatVoiceErrorCounterResource>**](VoiceChatVoiceErrorCounterResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v1_push_to_talk

> crate::models::VoiceChatPushToTalkResource get_voice_chat_v1_push_to_talk()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatPushToTalkResource**](VoiceChatPushToTalkResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_devices_capture

> Vec<crate::models::VoiceChatDeviceResource> get_voice_chat_v2_devices_capture()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VoiceChatDeviceResource>**](VoiceChatDeviceResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_devices_capture_permission

> crate::models::VoiceChatCaptureDevicePermissionStatus get_voice_chat_v2_devices_capture_permission()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatCaptureDevicePermissionStatus**](VoiceChatCaptureDevicePermissionStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_devices_render

> Vec<crate::models::VoiceChatDeviceResource> get_voice_chat_v2_devices_render()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VoiceChatDeviceResource>**](VoiceChatDeviceResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_sessions

> Vec<crate::models::VoiceChatSessionResource> get_voice_chat_v2_sessions()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VoiceChatSessionResource>**](VoiceChatSessionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_sessions_by_id

> crate::models::VoiceChatSessionResource get_voice_chat_v2_sessions_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::VoiceChatSessionResource**](VoiceChatSessionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_sessions_by_session_id_participants_by_participant_id

> crate::models::VoiceChatParticipantResource get_voice_chat_v2_sessions_by_session_id_participants_by_participant_id(session_id, participant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**participant_id** | **String** |  | [required] |

### Return type

[**crate::models::VoiceChatParticipantResource**](VoiceChatParticipantResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_settings

> crate::models::VoiceChatSettingsResource get_voice_chat_v2_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatSettingsResource**](VoiceChatSettingsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_voice_chat_v2_state

> crate::models::VoiceChatStateResource get_voice_chat_v2_state()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::VoiceChatStateResource**](VoiceChatStateResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voice_chat_v1_push_to_talk_check_available

> bool post_voice_chat_v1_push_to_talk_check_available(prompt)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prompt** | **i32** |  | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voice_chat_v1_sessions_by_id

> crate::models::VoiceChatSessionResource post_voice_chat_v1_sessions_by_id(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**crate::models::VoiceChatSessionResource**](VoiceChatSessionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voice_chat_v2_sessions

> crate::models::VoiceChatSessionResource post_voice_chat_v2_sessions(JWT)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**JWT** | **String** |  | [required] |

### Return type

[**crate::models::VoiceChatSessionResource**](VoiceChatSessionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_voice_chat_v2_sessions_by_id

> crate::models::VoiceChatSessionResource post_voice_chat_v2_sessions_by_id(id, JWT)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**JWT** | **String** |  | [required] |

### Return type

[**crate::models::VoiceChatSessionResource**](VoiceChatSessionResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voice_chat_v1_codec_settings

> put_voice_chat_v1_codec_settings(settings)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | [**VoiceChatUpdateCodecSettingsResource**](VoiceChatUpdateCodecSettingsResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voice_chat_v1_push_to_talk

> ::std::collections::HashMap<String, serde_json::Value> put_voice_chat_v1_push_to_talk(push_to_talk)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_to_talk** | [**VoiceChatPushToTalkResource**](VoiceChatPushToTalkResource.md) |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voice_chat_v2_devices_capture_prompt_for_permission

> put_voice_chat_v2_devices_capture_prompt_for_permission()


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


## put_voice_chat_v2_sessions_by_session_id_participants_by_participant_id

> put_voice_chat_v2_sessions_by_session_id_participants_by_participant_id(session_id, participant_id, participant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**session_id** | **String** |  | [required] |
**participant_id** | **String** |  | [required] |
**participant** | [**VoiceChatUpdateParticipantResource**](VoiceChatUpdateParticipantResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_voice_chat_v2_settings

> put_voice_chat_v2_settings(settings)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings** | [**VoiceChatUpdateSettingsResource**](VoiceChatUpdateSettingsResource.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

