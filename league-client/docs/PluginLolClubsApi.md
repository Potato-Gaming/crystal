# \PluginLolClubsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_lol_clubs_v1_clubs_by_club_key**](PluginLolClubsApi.md#delete_lol_clubs_v1_clubs_by_club_key) | **Delete** /lol-clubs/v1/clubs/{clubKey} | 
[**delete_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id**](PluginLolClubsApi.md#delete_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id) | **Delete** /lol-clubs/v1/clubs/{clubKey}/invitations/{summonerId} | 
[**delete_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id**](PluginLolClubsApi.md#delete_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id) | **Delete** /lol-clubs/v1/clubs/{clubKey}/members/{summonerId} | 
[**delete_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id**](PluginLolClubsApi.md#delete_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id) | **Delete** /lol-clubs/v1/clubs/{clubKey}/nominations/{summonerId} | 
[**delete_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id**](PluginLolClubsApi.md#delete_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id) | **Delete** /lol-clubs/v1/clubs/{clubKey}/promotions/{summonerId} | 
[**get_lol_clubs_v1_clubs**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs) | **Get** /lol-clubs/v1/clubs | 
[**get_lol_clubs_v1_clubs_by_club_key**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_by_club_key) | **Get** /lol-clubs/v1/clubs/{clubKey} | 
[**get_lol_clubs_v1_clubs_by_club_key_invitations**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_by_club_key_invitations) | **Get** /lol-clubs/v1/clubs/{clubKey}/invitations | 
[**get_lol_clubs_v1_clubs_by_club_key_members**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_by_club_key_members) | **Get** /lol-clubs/v1/clubs/{clubKey}/members | 
[**get_lol_clubs_v1_clubs_by_club_key_motd**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_by_club_key_motd) | **Get** /lol-clubs/v1/clubs/{clubKey}/motd | 
[**get_lol_clubs_v1_clubs_by_club_key_nominations**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_by_club_key_nominations) | **Get** /lol-clubs/v1/clubs/{clubKey}/nominations | 
[**get_lol_clubs_v1_clubs_invitations**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_invitations) | **Get** /lol-clubs/v1/clubs/invitations | 
[**get_lol_clubs_v1_clubs_membership**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_membership) | **Get** /lol-clubs/v1/clubs/membership | 
[**get_lol_clubs_v1_clubs_membership_preferences**](PluginLolClubsApi.md#get_lol_clubs_v1_clubs_membership_preferences) | **Get** /lol-clubs/v1/clubs/membership/preferences | 
[**patch_lol_clubs_v1_clubs_by_club_key**](PluginLolClubsApi.md#patch_lol_clubs_v1_clubs_by_club_key) | **Patch** /lol-clubs/v1/clubs/{clubKey} | 
[**patch_lol_clubs_v1_clubs_by_club_key_motd**](PluginLolClubsApi.md#patch_lol_clubs_v1_clubs_by_club_key_motd) | **Patch** /lol-clubs/v1/clubs/{clubKey}/motd | 
[**patch_lol_clubs_v1_clubs_invitations**](PluginLolClubsApi.md#patch_lol_clubs_v1_clubs_invitations) | **Patch** /lol-clubs/v1/clubs/invitations | 
[**patch_lol_clubs_v1_clubs_membership_preferences**](PluginLolClubsApi.md#patch_lol_clubs_v1_clubs_membership_preferences) | **Patch** /lol-clubs/v1/clubs/membership/preferences | 
[**post_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id) | **Post** /lol-clubs/v1/clubs/{clubKey}/invitations/{summonerId} | 
[**post_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id) | **Post** /lol-clubs/v1/clubs/{clubKey}/members/{summonerId} | 
[**post_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id) | **Post** /lol-clubs/v1/clubs/{clubKey}/nominations/{summonerId} | 
[**post_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id) | **Post** /lol-clubs/v1/clubs/{clubKey}/promotions/{summonerId} | 
[**post_lol_clubs_v1_clubs_by_club_key_view**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_by_club_key_view) | **Post** /lol-clubs/v1/clubs/{clubKey}/view | 
[**post_lol_clubs_v1_clubs_membership**](PluginLolClubsApi.md#post_lol_clubs_v1_clubs_membership) | **Post** /lol-clubs/v1/clubs/membership | 



## delete_lol_clubs_v1_clubs_by_club_key

> crate::models::LolClubsPlayerClubMembership delete_lol_clubs_v1_clubs_by_club_key(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClubMembership**](LolClubsPlayerClubMembership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id

> crate::models::LolClubsClubMemberLists delete_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id

> crate::models::LolClubsClubMemberLists delete_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id

> crate::models::LolClubsClubMemberLists delete_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id

> crate::models::LolClubsClubMemberLists delete_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs

> Vec<crate::models::LolClubsPlayerClub> get_lol_clubs_v1_clubs()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolClubsPlayerClub>**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_by_club_key

> crate::models::LolClubsPlayerClub get_lol_clubs_v1_clubs_by_club_key(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClub**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_by_club_key_invitations

> Vec<crate::models::LolClubsClubMember> get_lol_clubs_v1_clubs_by_club_key_invitations(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolClubsClubMember>**](LolClubsClubMember.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_by_club_key_members

> Vec<crate::models::LolClubsClubMember> get_lol_clubs_v1_clubs_by_club_key_members(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolClubsClubMember>**](LolClubsClubMember.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_by_club_key_motd

> String get_lol_clubs_v1_clubs_by_club_key_motd(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_by_club_key_nominations

> Vec<crate::models::LolClubsClubMember> get_lol_clubs_v1_clubs_by_club_key_nominations(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**Vec<crate::models::LolClubsClubMember>**](LolClubsClubMember.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_invitations

> Vec<crate::models::LolClubsClubInvite> get_lol_clubs_v1_clubs_invitations()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LolClubsClubInvite>**](LolClubsClubInvite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_membership

> crate::models::LolClubsPlayerClubMembership get_lol_clubs_v1_clubs_membership()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolClubsPlayerClubMembership**](LolClubsPlayerClubMembership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lol_clubs_v1_clubs_membership_preferences

> crate::models::LolClubsClubPreferences get_lol_clubs_v1_clubs_membership_preferences()


### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LolClubsClubPreferences**](LolClubsClubPreferences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_clubs_v1_clubs_by_club_key

> crate::models::LolClubsPlayerClub patch_lol_clubs_v1_clubs_by_club_key(club_key, tag)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**tag** | [**LolClubsClubTag**](LolClubsClubTag.md) |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClub**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_clubs_v1_clubs_by_club_key_motd

> crate::models::LolClubsPlayerClub patch_lol_clubs_v1_clubs_by_club_key_motd(club_key, motd)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**motd** | [**LolClubsClubMotd**](LolClubsClubMotd.md) |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClub**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_clubs_v1_clubs_invitations

> crate::models::LolClubsPlayerClubMembership patch_lol_clubs_v1_clubs_invitations(invitation)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitation** | [**LolClubsClubInvite**](LolClubsClubInvite.md) |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClubMembership**](LolClubsPlayerClubMembership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_lol_clubs_v1_clubs_membership_preferences

> crate::models::LolClubsClubPreferences patch_lol_clubs_v1_clubs_membership_preferences(preferences)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**preferences** | [**LolClubsClubPreferences**](LolClubsClubPreferences.md) |  | [required] |

### Return type

[**crate::models::LolClubsClubPreferences**](LolClubsClubPreferences.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id

> crate::models::LolClubsClubMemberLists post_lol_clubs_v1_clubs_by_club_key_invitations_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id

> crate::models::LolClubsPlayerClub post_lol_clubs_v1_clubs_by_club_key_members_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClub**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id

> crate::models::LolClubsClubMemberLists post_lol_clubs_v1_clubs_by_club_key_nominations_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id

> crate::models::LolClubsClubMemberLists post_lol_clubs_v1_clubs_by_club_key_promotions_by_summoner_id(club_key, summoner_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |
**summoner_id** | **i64** |  | [required] |

### Return type

[**crate::models::LolClubsClubMemberLists**](LolClubsClubMemberLists.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_by_club_key_view

> ::std::collections::HashMap<String, serde_json::Value> post_lol_clubs_v1_clubs_by_club_key_view(club_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**club_key** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_lol_clubs_v1_clubs_membership

> crate::models::LolClubsPlayerClub post_lol_clubs_v1_clubs_membership(name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | [**LolClubsClubName**](LolClubsClubName.md) |  | [required] |

### Return type

[**crate::models::LolClubsPlayerClub**](LolClubsPlayerClub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/vnd.api+json, application/x-yaml, application/x-msgpack, application/octet-stream, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json, application/x-yaml, application/x-msgpack

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

