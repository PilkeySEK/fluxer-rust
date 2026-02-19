# \GuildsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_guild_member_role**](GuildsApi.md#add_guild_member_role) | **PUT** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | Add role to guild member
[**ban_guild_member2**](GuildsApi.md#ban_guild_member2) | **PUT** /guilds/{guild_id}/bans/{user_id} | Ban guild member
[**bulk_create_guild_emojis**](GuildsApi.md#bulk_create_guild_emojis) | **POST** /guilds/{guild_id}/emojis/bulk | Bulk create guild emojis
[**bulk_create_guild_stickers**](GuildsApi.md#bulk_create_guild_stickers) | **POST** /guilds/{guild_id}/stickers/bulk | Bulk create guild stickers
[**create_guild**](GuildsApi.md#create_guild) | **POST** /guilds | Create guild
[**create_guild_channel**](GuildsApi.md#create_guild_channel) | **POST** /guilds/{guild_id}/channels | Create guild channel
[**create_guild_emoji**](GuildsApi.md#create_guild_emoji) | **POST** /guilds/{guild_id}/emojis | Create guild emoji
[**create_guild_role**](GuildsApi.md#create_guild_role) | **POST** /guilds/{guild_id}/roles | Create guild role
[**create_guild_sticker**](GuildsApi.md#create_guild_sticker) | **POST** /guilds/{guild_id}/stickers | Create guild sticker
[**delete_guild2**](GuildsApi.md#delete_guild2) | **POST** /guilds/{guild_id}/delete | Delete guild
[**delete_guild_emoji**](GuildsApi.md#delete_guild_emoji) | **DELETE** /guilds/{guild_id}/emojis/{emoji_id} | Delete guild emoji
[**delete_guild_role**](GuildsApi.md#delete_guild_role) | **DELETE** /guilds/{guild_id}/roles/{role_id} | Delete guild role
[**delete_guild_sticker**](GuildsApi.md#delete_guild_sticker) | **DELETE** /guilds/{guild_id}/stickers/{sticker_id} | Delete guild sticker
[**get_current_guild_member**](GuildsApi.md#get_current_guild_member) | **GET** /guilds/{guild_id}/members/@me | Get current user guild member
[**get_guild**](GuildsApi.md#get_guild) | **GET** /guilds/{guild_id} | Get guild information
[**get_guild_member**](GuildsApi.md#get_guild_member) | **GET** /guilds/{guild_id}/members/{user_id} | Get guild member by user ID
[**get_guild_vanity_url**](GuildsApi.md#get_guild_vanity_url) | **GET** /guilds/{guild_id}/vanity-url | Get guild vanity URL
[**leave_guild**](GuildsApi.md#leave_guild) | **DELETE** /users/@me/guilds/{guild_id} | Leave guild
[**list_guild_audit_logs**](GuildsApi.md#list_guild_audit_logs) | **GET** /guilds/{guild_id}/audit-logs | List guild audit logs
[**list_guild_bans**](GuildsApi.md#list_guild_bans) | **GET** /guilds/{guild_id}/bans | List guild bans
[**list_guild_channels**](GuildsApi.md#list_guild_channels) | **GET** /guilds/{guild_id}/channels | List guild channels
[**list_guild_emojis2**](GuildsApi.md#list_guild_emojis2) | **GET** /guilds/{guild_id}/emojis | List guild emojis
[**list_guild_members2**](GuildsApi.md#list_guild_members2) | **GET** /guilds/{guild_id}/members | List guild members
[**list_guild_roles**](GuildsApi.md#list_guild_roles) | **GET** /guilds/{guild_id}/roles | List guild roles
[**list_guild_stickers2**](GuildsApi.md#list_guild_stickers2) | **GET** /guilds/{guild_id}/stickers | List guild stickers
[**list_guilds**](GuildsApi.md#list_guilds) | **GET** /users/@me/guilds | List current user guilds
[**remove_guild_member**](GuildsApi.md#remove_guild_member) | **DELETE** /guilds/{guild_id}/members/{user_id} | Remove guild member
[**remove_guild_member_role**](GuildsApi.md#remove_guild_member_role) | **DELETE** /guilds/{guild_id}/members/{user_id}/roles/{role_id} | Remove role from guild member
[**reset_role_hoist_positions**](GuildsApi.md#reset_role_hoist_positions) | **DELETE** /guilds/{guild_id}/roles/hoist-positions | Reset role hoist positions
[**search_guild_members**](GuildsApi.md#search_guild_members) | **POST** /guilds/{guild_id}/members-search | Search guild members
[**toggle_detached_banner**](GuildsApi.md#toggle_detached_banner) | **PATCH** /guilds/{guild_id}/detached-banner | Toggle detached banner
[**toggle_text_channel_flexible_names**](GuildsApi.md#toggle_text_channel_flexible_names) | **PATCH** /guilds/{guild_id}/text-channel-flexible-names | Toggle text channel flexible names
[**transfer_guild_ownership2**](GuildsApi.md#transfer_guild_ownership2) | **POST** /guilds/{guild_id}/transfer-ownership | Transfer guild ownership
[**unban_guild_member**](GuildsApi.md#unban_guild_member) | **DELETE** /guilds/{guild_id}/bans/{user_id} | Unban guild member
[**update_current_guild_member**](GuildsApi.md#update_current_guild_member) | **PATCH** /guilds/{guild_id}/members/@me | Update current user guild member
[**update_guild**](GuildsApi.md#update_guild) | **PATCH** /guilds/{guild_id} | Update guild settings
[**update_guild_channel_positions**](GuildsApi.md#update_guild_channel_positions) | **PATCH** /guilds/{guild_id}/channels | Update channel positions
[**update_guild_emoji**](GuildsApi.md#update_guild_emoji) | **PATCH** /guilds/{guild_id}/emojis/{emoji_id} | Update guild emoji
[**update_guild_member**](GuildsApi.md#update_guild_member) | **PATCH** /guilds/{guild_id}/members/{user_id} | Update guild member
[**update_guild_role**](GuildsApi.md#update_guild_role) | **PATCH** /guilds/{guild_id}/roles/{role_id} | Update guild role
[**update_guild_role_positions**](GuildsApi.md#update_guild_role_positions) | **PATCH** /guilds/{guild_id}/roles | Update role positions
[**update_guild_sticker**](GuildsApi.md#update_guild_sticker) | **PATCH** /guilds/{guild_id}/stickers/{sticker_id} | Update guild sticker
[**update_guild_vanity_url**](GuildsApi.md#update_guild_vanity_url) | **PATCH** /guilds/{guild_id}/vanity-url | Update guild vanity URL
[**update_role_hoist_positions**](GuildsApi.md#update_role_hoist_positions) | **PATCH** /guilds/{guild_id}/roles/hoist-positions | Update role hoist positions



## add_guild_member_role

> add_guild_member_role(guild_id, user_id, role_id)
Add role to guild member

Add role to guild member. Requires manage_roles permission. Grants the specified role to the user in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |
**role_id** | **String** | The ID of the role | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ban_guild_member2

> ban_guild_member2(guild_id, user_id, guild_ban_create_request)
Ban guild member

Ban guild member. Requires ban_members permission. Prevents user from joining; optionally deletes recent messages and sets ban expiry duration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |
**guild_ban_create_request** | [**GuildBanCreateRequest**](GuildBanCreateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_guild_emojis

> models::GuildEmojiBulkCreateResponse bulk_create_guild_emojis(guild_id, guild_emoji_bulk_create_request)
Bulk create guild emojis

Bulk create guild emojis. Requires manage_emojis permission. Creates multiple emojis in a single request for efficiency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_emoji_bulk_create_request** | [**GuildEmojiBulkCreateRequest**](GuildEmojiBulkCreateRequest.md) |  | [required] |

### Return type

[**models::GuildEmojiBulkCreateResponse**](GuildEmojiBulkCreateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_guild_stickers

> models::GuildStickerBulkCreateResponse bulk_create_guild_stickers(guild_id, guild_sticker_bulk_create_request)
Bulk create guild stickers

Bulk create guild stickers. Requires manage_emojis permission. Creates multiple stickers in a single request for efficiency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_sticker_bulk_create_request** | [**GuildStickerBulkCreateRequest**](GuildStickerBulkCreateRequest.md) |  | [required] |

### Return type

[**models::GuildStickerBulkCreateResponse**](GuildStickerBulkCreateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild

> models::GuildResponse create_guild(guild_create_request)
Create guild

Only authenticated users can create guilds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_create_request** | [**GuildCreateRequest**](GuildCreateRequest.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_channel

> models::ChannelResponse create_guild_channel(guild_id, channel_create_request)
Create guild channel

Create guild channel. Requires manage_channels permission. Creates a new text, voice, or category channel in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**channel_create_request** | [**ChannelCreateRequest**](ChannelCreateRequest.md) |  | [required] |

### Return type

[**models::ChannelResponse**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_emoji

> models::GuildEmojiResponse create_guild_emoji(guild_id, guild_emoji_create_request)
Create guild emoji

Create guild emoji. Requires manage_emojis permission. Uploads and registers a new custom emoji for the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_emoji_create_request** | [**GuildEmojiCreateRequest**](GuildEmojiCreateRequest.md) |  | [required] |

### Return type

[**models::GuildEmojiResponse**](GuildEmojiResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_role

> models::GuildRoleResponse create_guild_role(guild_id, guild_role_create_request)
Create guild role

Create guild role. Requires manage_roles permission. Creates a new role with specified name, permissions, and color.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_role_create_request** | [**GuildRoleCreateRequest**](GuildRoleCreateRequest.md) |  | [required] |

### Return type

[**models::GuildRoleResponse**](GuildRoleResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_guild_sticker

> models::GuildStickerResponse create_guild_sticker(guild_id, guild_sticker_create_request)
Create guild sticker

Create guild sticker. Requires manage_emojis permission. Uploads a new sticker with name, description, and tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_sticker_create_request** | [**GuildStickerCreateRequest**](GuildStickerCreateRequest.md) |  | [required] |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild2

> delete_guild2(guild_id, guild_delete_request)
Delete guild

Only guild owner can delete. Requires sudo mode verification (MFA). Permanently deletes the guild and all associated data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_delete_request** | [**GuildDeleteRequest**](GuildDeleteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_emoji

> delete_guild_emoji(guild_id, emoji_id, purge)
Delete guild emoji

Delete guild emoji. Requires manage_emojis permission. Removes a custom emoji from the guild; optionally purges all references.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**emoji_id** | **String** | The ID of the emoji | [required] |
**purge** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_role

> delete_guild_role(guild_id, role_id)
Delete guild role

Delete guild role. Requires manage_roles permission. Permanently removes the role from the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**role_id** | **String** | The ID of the role | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_guild_sticker

> delete_guild_sticker(guild_id, sticker_id, purge)
Delete guild sticker

Delete guild sticker. Requires manage_emojis permission. Removes a sticker from the guild; optionally purges all references.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**sticker_id** | **String** | The ID of the sticker | [required] |
**purge** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_guild_member

> models::GuildMemberResponse get_current_guild_member(guild_id)
Get current user guild member

Get current user guild member. Returns the member information for the authenticated user in the specified guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild

> models::GuildResponse get_guild(guild_id)
Get guild information

User must be a member of the guild to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_member

> models::GuildMemberResponse get_guild_member(guild_id, user_id)
Get guild member by user ID

Get guild member by user ID. Returns member information including roles, nickname, and join date for the specified user in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_guild_vanity_url

> models::GuildVanityUrlResponse get_guild_vanity_url(guild_id)
Get guild vanity URL

Returns the custom invite code for the guild if configured.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::GuildVanityUrlResponse**](GuildVanityURLResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## leave_guild

> leave_guild(guild_id)
Leave guild

Removes the current user from the specified guild membership.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_audit_logs

> models::GuildAuditLogListResponse list_guild_audit_logs(guild_id, limit, before, after, user_id, action_type)
List guild audit logs

List guild audit logs. Only default users can access. Requires view_audit_logs permission. Returns guild activity history with pagination and action filtering.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**limit** | Option<**i32**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**action_type** | Option<[**AuditLogActionType**](AuditLogActionType.md)> |  |  |

### Return type

[**models::GuildAuditLogListResponse**](GuildAuditLogListResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_bans

> Vec<models::GuildBanResponse> list_guild_bans(guild_id)
List guild bans

List guild bans. Requires ban_members permission. Returns all banned users for the guild including ban reasons and expiry times.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::GuildBanResponse>**](GuildBanResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_channels

> Vec<models::ChannelResponse> list_guild_channels(guild_id)
List guild channels

List guild channels. Returns all channels in the guild that the user has access to view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::ChannelResponse>**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_emojis2

> Vec<models::GuildEmojiWithUserResponse> list_guild_emojis2(guild_id)
List guild emojis

List guild emojis. Returns all custom emojis for the guild including metadata about creators and timestamps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::GuildEmojiWithUserResponse>**](GuildEmojiWithUserResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_members2

> Vec<models::GuildMemberResponse> list_guild_members2(guild_id, limit, after)
List guild members

List guild members. Supports pagination with limit and after cursor. Returns member information for the specified guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**limit** | Option<**i32**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::GuildMemberResponse>**](GuildMemberResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_roles

> Vec<models::GuildRoleResponse> list_guild_roles(guild_id)
List guild roles

List guild roles. Returns all roles defined in the guild including their permissions and settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::GuildRoleResponse>**](GuildRoleResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_stickers2

> Vec<models::GuildStickerWithUserResponse> list_guild_stickers2(guild_id)
List guild stickers

List guild stickers. Returns all custom stickers for the guild including metadata about creators, descriptions, and tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::GuildStickerWithUserResponse>**](GuildStickerWithUserResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guilds

> Vec<models::GuildResponse> list_guilds(before, after, limit, with_counts)
List current user guilds

Requires guilds OAuth scope if using bearer token. Returns all guilds the user is a member of.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**with_counts** | Option<**String**> |  |  |

### Return type

[**Vec<models::GuildResponse>**](GuildResponse.md)

### Authorization

[oauth2Token](../README.md#oauth2Token), [sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_guild_member

> remove_guild_member(guild_id, user_id)
Remove guild member

Remove guild member. Requires kick_members permission. Removes the specified user from the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_guild_member_role

> remove_guild_member_role(guild_id, user_id, role_id)
Remove role from guild member

Remove role from guild member. Requires manage_roles permission. Revokes the specified role from the user in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |
**role_id** | **String** | The ID of the role | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_role_hoist_positions

> reset_role_hoist_positions(guild_id)
Reset role hoist positions

Reset role hoist positions. Requires manage_roles permission. Clears all hoist position assignments for roles in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_guild_members

> models::GuildMemberSearchResponse search_guild_members(guild_id, guild_member_search_request)
Search guild members

Search and filter guild members with pagination support.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_member_search_request** | [**GuildMemberSearchRequest**](GuildMemberSearchRequest.md) |  | [required] |

### Return type

[**models::GuildMemberSearchResponse**](GuildMemberSearchResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_detached_banner

> models::GuildResponse toggle_detached_banner(guild_id, enabled_toggle_request)
Toggle detached banner

Requires manage_guild permission. Enables or disables independent banner display configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**enabled_toggle_request** | [**EnabledToggleRequest**](EnabledToggleRequest.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## toggle_text_channel_flexible_names

> models::GuildResponse toggle_text_channel_flexible_names(guild_id, enabled_toggle_request)
Toggle text channel flexible names

Requires manage_guild permission. Allows or disables flexible naming for text channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**enabled_toggle_request** | [**EnabledToggleRequest**](EnabledToggleRequest.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_guild_ownership2

> models::GuildResponse transfer_guild_ownership2(guild_id, guild_transfer_ownership_request)
Transfer guild ownership

Transfer guild ownership. Only current owner can transfer. Requires sudo mode verification (MFA). Transfers all guild permissions to a new owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_transfer_ownership_request** | [**GuildTransferOwnershipRequest**](GuildTransferOwnershipRequest.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unban_guild_member

> unban_guild_member(guild_id, user_id)
Unban guild member

Unban guild member. Requires ban_members permission. Removes ban and allows user to rejoin the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_guild_member

> models::GuildMemberResponse update_current_guild_member(guild_id, my_guild_member_update_request)
Update current user guild member

Update current user guild member. User can modify their own nickname within the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**my_guild_member_update_request** | [**MyGuildMemberUpdateRequest**](MyGuildMemberUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild

> models::GuildResponse update_guild(guild_id, guild_update_request)
Update guild settings

Requires manage_guild permission. Updates guild name, description, icon, banner, and other configuration options.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_update_request** | [**GuildUpdateRequest**](GuildUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildResponse**](GuildResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_channel_positions

> update_guild_channel_positions(guild_id, channel_position_update_request_inner)
Update channel positions

Update channel positions. Requires manage_channels permission. Reorders channels and optionally changes parent categories and permission locks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**channel_position_update_request_inner** | [**Vec<models::ChannelPositionUpdateRequestInner>**](ChannelPositionUpdateRequestInner.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_emoji

> models::GuildEmojiResponse update_guild_emoji(guild_id, emoji_id, guild_emoji_update_request)
Update guild emoji

Update guild emoji. Requires manage_emojis permission. Renames or updates properties of an existing emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**emoji_id** | **String** | The ID of the emoji | [required] |
**guild_emoji_update_request** | [**GuildEmojiUpdateRequest**](GuildEmojiUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildEmojiResponse**](GuildEmojiResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_member

> models::GuildMemberResponse update_guild_member(guild_id, user_id, guild_member_update_request)
Update guild member

Update guild member. Requires manage_members permission. Can modify member nickname, voice state, and other member properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**user_id** | **String** | The ID of the user | [required] |
**guild_member_update_request** | [**GuildMemberUpdateRequest**](GuildMemberUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildMemberResponse**](GuildMemberResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_role

> models::GuildRoleResponse update_guild_role(guild_id, role_id, guild_role_update_request)
Update guild role

Update guild role. Requires manage_roles permission. Modifies role name, permissions, color, and other settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**role_id** | **String** | The ID of the role | [required] |
**guild_role_update_request** | [**GuildRoleUpdateRequest**](GuildRoleUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildRoleResponse**](GuildRoleResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_role_positions

> update_guild_role_positions(guild_id, guild_role_position_item)
Update role positions

Update role positions. Requires manage_roles permission. Reorders roles to change their hierarchy and permission precedence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_role_position_item** | [**Vec<models::GuildRolePositionItem>**](GuildRolePositionItem.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_sticker

> models::GuildStickerResponse update_guild_sticker(guild_id, sticker_id, guild_sticker_update_request)
Update guild sticker

Update guild sticker. Requires manage_emojis permission. Updates sticker name, description, or tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**sticker_id** | **String** | The ID of the sticker | [required] |
**guild_sticker_update_request** | [**GuildStickerUpdateRequest**](GuildStickerUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_guild_vanity_url

> models::GuildVanityUrlUpdateResponse update_guild_vanity_url(guild_id, guild_vanity_url_update_request)
Update guild vanity URL

Only default users can set vanity URLs. Requires manage_guild permission. Sets or removes a custom invite code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_vanity_url_update_request** | [**GuildVanityUrlUpdateRequest**](GuildVanityUrlUpdateRequest.md) |  | [required] |

### Return type

[**models::GuildVanityUrlUpdateResponse**](GuildVanityURLUpdateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_hoist_positions

> update_role_hoist_positions(guild_id, guild_role_hoist_position_item)
Update role hoist positions

Update role hoist positions. Requires manage_roles permission. Sets the display priority for hoisted (separated) roles in the member list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**guild_role_hoist_position_item** | [**Vec<models::GuildRoleHoistPositionItem>**](GuildRoleHoistPositionItem.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

