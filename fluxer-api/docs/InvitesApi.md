# \InvitesApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invite**](InvitesApi.md#accept_invite) | **POST** /invites/{invite_code} | Accept invite
[**create_channel_invite**](InvitesApi.md#create_channel_invite) | **POST** /channels/{channel_id}/invites | Create channel invite
[**create_pack_invite**](InvitesApi.md#create_pack_invite) | **POST** /packs/{pack_id}/invites | Create pack invite
[**delete_invite**](InvitesApi.md#delete_invite) | **DELETE** /invites/{invite_code} | Delete invite
[**get_invite**](InvitesApi.md#get_invite) | **GET** /invites/{invite_code} | Get invite information
[**list_channel_invites**](InvitesApi.md#list_channel_invites) | **GET** /channels/{channel_id}/invites | List channel invites
[**list_guild_invites**](InvitesApi.md#list_guild_invites) | **GET** /guilds/{guild_id}/invites | List guild invites
[**list_pack_invites**](InvitesApi.md#list_pack_invites) | **GET** /packs/{pack_id}/invites | List pack invites



## accept_invite

> models::InviteResponseSchema accept_invite(invite_code)
Accept invite

Accepts an invite using its code, adding the authenticated user to the corresponding guild, pack, or other entity. The invite usage count is incremented, and if it reaches its maximum usage limit or expiration, the invite is automatically revoked. Returns the accepted invite details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_code** | **String** | The invite code | [required] |

### Return type

[**models::InviteResponseSchema**](InviteResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_channel_invite

> models::InviteMetadataResponseSchema create_channel_invite(channel_id, channel_invite_create_request)
Create channel invite

Creates a new invite for the specified channel with optional parameters such as maximum age, maximum uses, and temporary membership settings. The authenticated user must have permission to create invites for the channel. Returns the created invite with full metadata including usage statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**channel_invite_create_request** | [**ChannelInviteCreateRequest**](ChannelInviteCreateRequest.md) |  | [required] |

### Return type

[**models::InviteMetadataResponseSchema**](InviteMetadataResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pack_invite

> models::InviteMetadataResponseSchema create_pack_invite(pack_id, pack_invite_create_request)
Create pack invite

Creates a new invite for the specified pack with optional parameters such as maximum age and maximum uses. The authenticated user must have permission to create invites for the pack and must be a default (non-bot) user. Returns the created invite with full metadata including usage statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**pack_invite_create_request** | [**PackInviteCreateRequest**](PackInviteCreateRequest.md) |  | [required] |

### Return type

[**models::InviteMetadataResponseSchema**](InviteMetadataResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invite

> delete_invite(invite_code)
Delete invite

Permanently deletes an invite by its code, preventing any further usage. The authenticated user must have permission to manage invites for the guild, channel, or pack associated with the invite. This action can be logged in the audit log if an X-Audit-Log-Reason header is provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_code** | **String** | The invite code | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invite

> models::InviteResponseSchema get_invite(invite_code)
Get invite information

Fetches detailed information about an invite using its code, including the guild, channel, or pack it belongs to and metadata such as expiration and usage limits. This endpoint does not require authentication and does not consume the invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invite_code** | **String** | The invite code | [required] |

### Return type

[**models::InviteResponseSchema**](InviteResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channel_invites

> Vec<models::InviteMetadataResponseSchema> list_channel_invites(channel_id)
List channel invites

Retrieves all currently active invites for the specified channel, including invite codes, creators, expiration times, and usage statistics. The authenticated user must have permission to manage invites for the channel. Returns an array of invite metadata objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**Vec<models::InviteMetadataResponseSchema>**](InviteMetadataResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_invites

> Vec<models::InviteMetadataResponseSchema> list_guild_invites(guild_id)
List guild invites

Retrieves all currently active invites across all channels in the specified guild, including invite codes, creators, expiration times, and usage statistics. The authenticated user must have permission to manage invites for the guild. Returns an array of invite metadata objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::InviteMetadataResponseSchema>**](InviteMetadataResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pack_invites

> Vec<models::InviteMetadataResponseSchema> list_pack_invites(pack_id)
List pack invites

Retrieves all currently active invites for the specified pack, including invite codes, creators, expiration times, and usage statistics. The authenticated user must have permission to manage invites for the pack and must be a default (non-bot) user. Returns an array of invite metadata objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

[**Vec<models::InviteMetadataResponseSchema>**](InviteMetadataResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

