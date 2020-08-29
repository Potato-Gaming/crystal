# \PluginLolPremadeVoiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_premade_voice_v1_mic_test**](PluginLolPremadeVoiceApi.md#delete_lol_premade_voice_v1_mic_test) | **Delete** /lol-premade-voice/v1/mic-test | 
[**delete_lol_premade_voice_v1_session**](PluginLolPremadeVoiceApi.md#delete_lol_premade_voice_v1_session) | **Delete** /lol-premade-voice/v1/session | 
[**get_lol_premade_voice_v1_availability**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_availability) | **Get** /lol-premade-voice/v1/availability | 
[**get_lol_premade_voice_v1_capturedevices**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_capturedevices) | **Get** /lol-premade-voice/v1/capturedevices | 
[**get_lol_premade_voice_v1_first_experience**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_first_experience) | **Get** /lol-premade-voice/v1/first-experience | 
[**get_lol_premade_voice_v1_mic_test**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_mic_test) | **Get** /lol-premade-voice/v1/mic-test | 
[**get_lol_premade_voice_v1_participant_records**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_participant_records) | **Get** /lol-premade-voice/v1/participant-records | 
[**get_lol_premade_voice_v1_participants**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_participants) | **Get** /lol-premade-voice/v1/participants | 
[**get_lol_premade_voice_v1_settings**](PluginLolPremadeVoiceApi.md#get_lol_premade_voice_v1_settings) | **Get** /lol-premade-voice/v1/settings | 
[**post_lol_premade_voice_v1_first_experience_game**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_first_experience_game) | **Post** /lol-premade-voice/v1/first-experience/game | 
[**post_lol_premade_voice_v1_first_experience_lcu**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_first_experience_lcu) | **Post** /lol-premade-voice/v1/first-experience/lcu | 
[**post_lol_premade_voice_v1_first_experience_reset**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_first_experience_reset) | **Post** /lol-premade-voice/v1/first-experience/reset | 
[**post_lol_premade_voice_v1_game_client_updated_ptt_key**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_game_client_updated_ptt_key) | **Post** /lol-premade-voice/v1/gameClientUpdatedPTTKey | 
[**post_lol_premade_voice_v1_mic_test**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_mic_test) | **Post** /lol-premade-voice/v1/mic-test | 
[**post_lol_premade_voice_v1_push_to_talk_check_available**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_push_to_talk_check_available) | **Post** /lol-premade-voice/v1/push-to-talk/check-available | 
[**post_lol_premade_voice_v1_session**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_session) | **Post** /lol-premade-voice/v1/session | 
[**post_lol_premade_voice_v1_settings_reset**](PluginLolPremadeVoiceApi.md#post_lol_premade_voice_v1_settings_reset) | **Post** /lol-premade-voice/v1/settings/reset | 
[**put_lol_premade_voice_v1_capturedevices**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_capturedevices) | **Put** /lol-premade-voice/v1/capturedevices | 
[**put_lol_premade_voice_v1_participants_by_puuid_mute**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_participants_by_puuid_mute) | **Put** /lol-premade-voice/v1/participants/{puuid}/mute | 
[**put_lol_premade_voice_v1_participants_by_puuid_volume**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_participants_by_puuid_volume) | **Put** /lol-premade-voice/v1/participants/{puuid}/volume | 
[**put_lol_premade_voice_v1_self_activation_sensitivity**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_self_activation_sensitivity) | **Put** /lol-premade-voice/v1/self/activationSensitivity | 
[**put_lol_premade_voice_v1_self_input_mode**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_self_input_mode) | **Put** /lol-premade-voice/v1/self/inputMode | 
[**put_lol_premade_voice_v1_self_mic_level**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_self_mic_level) | **Put** /lol-premade-voice/v1/self/micLevel | 
[**put_lol_premade_voice_v1_self_mute**](PluginLolPremadeVoiceApi.md#put_lol_premade_voice_v1_self_mute) | **Put** /lol-premade-voice/v1/self/mute | 



## delete_lol_premade_voice_v1_mic_test

> ::std::collections::HashMap<String, serde_json::Value> delete_lol_premade_voice_v1_mic_test()


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


## delete_lol_premade_voice_v1_session

> delete_lol_premade_voice_v1_session()


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


## get_lol_premade_voice_v1_availability

> crate::models::LolPremadeVoiceVoiceAvailability get_lol_premade_voice_v1_availability()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPremadeVoiceVoiceAvailability**](LolPremadeVoiceVoiceAvailability.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_capturedevices

> Vec<crate::models::LolPremadeVoiceDeviceResource> get_lol_premade_voice_v1_capturedevices()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPremadeVoiceDeviceResource>**](LolPremadeVoiceDeviceResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_first_experience

> crate::models::LolPremadeVoiceFirstExperience get_lol_premade_voice_v1_first_experience()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPremadeVoiceFirstExperience**](LolPremadeVoiceFirstExperience.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_mic_test

> crate::models::LolPremadeVoiceAudioPropertiesResource get_lol_premade_voice_v1_mic_test()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPremadeVoiceAudioPropertiesResource**](LolPremadeVoiceAudioPropertiesResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_participant_records

> Vec<crate::models::LolPremadeVoicePremadeVoiceParticipantDto> get_lol_premade_voice_v1_participant_records()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPremadeVoicePremadeVoiceParticipantDto>**](LolPremadeVoicePremadeVoiceParticipantDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_participants

> Vec<crate::models::LolPremadeVoicePremadeVoiceParticipantDto> get_lol_premade_voice_v1_participants()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolPremadeVoicePremadeVoiceParticipantDto>**](LolPremadeVoicePremadeVoiceParticipantDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_premade_voice_v1_settings

> crate::models::LolPremadeVoiceSettingsResource get_lol_premade_voice_v1_settings()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolPremadeVoiceSettingsResource**](LolPremadeVoiceSettingsResource.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_premade_voice_v1_first_experience_game

> post_lol_premade_voice_v1_first_experience_game()


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


## post_lol_premade_voice_v1_first_experience_lcu

> post_lol_premade_voice_v1_first_experience_lcu()


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


## post_lol_premade_voice_v1_first_experience_reset

> post_lol_premade_voice_v1_first_experience_reset()


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


## post_lol_premade_voice_v1_game_client_updated_ptt_key

> post_lol_premade_voice_v1_game_client_updated_ptt_key(new_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_key** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_premade_voice_v1_mic_test

> ::std::collections::HashMap<String, serde_json::Value> post_lol_premade_voice_v1_mic_test()


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


## post_lol_premade_voice_v1_push_to_talk_check_available

> bool post_lol_premade_voice_v1_push_to_talk_check_available(prompt)


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


## post_lol_premade_voice_v1_session

> post_lol_premade_voice_v1_session()


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


## post_lol_premade_voice_v1_settings_reset

> post_lol_premade_voice_v1_settings_reset()


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


## put_lol_premade_voice_v1_capturedevices

> put_lol_premade_voice_v1_capturedevices(handle)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**handle** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_participants_by_puuid_mute

> put_lol_premade_voice_v1_participants_by_puuid_mute(puuid, muted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**muted** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_participants_by_puuid_volume

> put_lol_premade_voice_v1_participants_by_puuid_volume(puuid, volume)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**puuid** | **String** |  | [required] |
**volume** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_self_activation_sensitivity

> put_lol_premade_voice_v1_self_activation_sensitivity(sensitivity)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sensitivity** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_self_input_mode

> put_lol_premade_voice_v1_self_input_mode(input_mode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**input_mode** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_self_mic_level

> put_lol_premade_voice_v1_self_mic_level(mic_level)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mic_level** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_lol_premade_voice_v1_self_mute

> put_lol_premade_voice_v1_self_mute(muted)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**muted** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

