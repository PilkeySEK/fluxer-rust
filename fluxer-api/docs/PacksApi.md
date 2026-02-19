# \PacksApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_pack_emojis**](PacksApi.md#bulk_create_pack_emojis) | **POST** /packs/emojis/{pack_id}/bulk | Bulk create pack emojis
[**bulk_create_pack_stickers**](PacksApi.md#bulk_create_pack_stickers) | **POST** /packs/stickers/{pack_id}/bulk | Bulk create pack stickers
[**create_pack**](PacksApi.md#create_pack) | **POST** /packs/{pack_type} | Create pack
[**create_pack_emoji**](PacksApi.md#create_pack_emoji) | **POST** /packs/emojis/{pack_id} | Create pack emoji
[**create_pack_sticker**](PacksApi.md#create_pack_sticker) | **POST** /packs/stickers/{pack_id} | Create pack sticker
[**delete_pack**](PacksApi.md#delete_pack) | **DELETE** /packs/{pack_id} | Delete pack
[**delete_pack_emoji**](PacksApi.md#delete_pack_emoji) | **DELETE** /packs/emojis/{pack_id}/{emoji_id} | Delete pack emoji
[**delete_pack_sticker**](PacksApi.md#delete_pack_sticker) | **DELETE** /packs/stickers/{pack_id}/{sticker_id} | Delete pack sticker
[**install_pack**](PacksApi.md#install_pack) | **POST** /packs/{pack_id}/install | Install pack
[**list_pack_emojis**](PacksApi.md#list_pack_emojis) | **GET** /packs/emojis/{pack_id} | List pack emojis
[**list_pack_stickers**](PacksApi.md#list_pack_stickers) | **GET** /packs/stickers/{pack_id} | List pack stickers
[**list_user_packs**](PacksApi.md#list_user_packs) | **GET** /packs | List user packs
[**uninstall_pack**](PacksApi.md#uninstall_pack) | **DELETE** /packs/{pack_id}/install | Uninstall pack
[**update_pack**](PacksApi.md#update_pack) | **PATCH** /packs/{pack_id} | Update pack
[**update_pack_emoji**](PacksApi.md#update_pack_emoji) | **PATCH** /packs/emojis/{pack_id}/{emoji_id} | Update pack emoji
[**update_pack_sticker**](PacksApi.md#update_pack_sticker) | **PATCH** /packs/stickers/{pack_id}/{sticker_id} | Update pack sticker



## bulk_create_pack_emojis

> models::GuildEmojiBulkCreateResponse bulk_create_pack_emojis(pack_id, guild_emoji_bulk_create_request)
Bulk create pack emojis

Creates multiple emojis within the specified pack in a single bulk operation. Accepts an array of emoji definitions, each containing name and image data. Returns a response containing all successfully created emojis.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**guild_emoji_bulk_create_request** | [**GuildEmojiBulkCreateRequest**](GuildEmojiBulkCreateRequest.md) |  | [required] |

### Return type

[**models::GuildEmojiBulkCreateResponse**](GuildEmojiBulkCreateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_create_pack_stickers

> models::GuildStickerBulkCreateResponse bulk_create_pack_stickers(pack_id, guild_sticker_bulk_create_request)
Bulk create pack stickers

Creates multiple stickers within the specified pack in a single bulk operation. Accepts an array of sticker definitions, each containing name, description, tags, and image data. Returns a response containing all successfully created stickers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**guild_sticker_bulk_create_request** | [**GuildStickerBulkCreateRequest**](GuildStickerBulkCreateRequest.md) |  | [required] |

### Return type

[**models::GuildStickerBulkCreateResponse**](GuildStickerBulkCreateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pack

> models::PackSummaryResponse create_pack(pack_type, pack_create_request)
Create pack

Creates a new emoji or sticker pack owned by the authenticated user. The pack type is specified in the path parameter and can be either \"emoji\" or \"sticker\". Returns the newly created pack with its metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_type** | **String** | The pack type | [required] |
**pack_create_request** | [**PackCreateRequest**](PackCreateRequest.md) |  | [required] |

### Return type

[**models::PackSummaryResponse**](PackSummaryResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pack_emoji

> models::GuildEmojiResponse create_pack_emoji(pack_id, guild_emoji_create_request)
Create pack emoji

Creates a new emoji within the specified pack. Requires the pack ID in the path and emoji metadata (name and image data) in the request body. Returns the newly created emoji with its generated ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**guild_emoji_create_request** | [**GuildEmojiCreateRequest**](GuildEmojiCreateRequest.md) |  | [required] |

### Return type

[**models::GuildEmojiResponse**](GuildEmojiResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pack_sticker

> models::GuildStickerResponse create_pack_sticker(pack_id, guild_sticker_create_request)
Create pack sticker

Creates a new sticker within the specified pack. Requires the pack ID in the path and sticker metadata (name, description, tags, and image data) in the request body. Returns the newly created sticker with its generated ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**guild_sticker_create_request** | [**GuildStickerCreateRequest**](GuildStickerCreateRequest.md) |  | [required] |

### Return type

[**models::GuildStickerResponse**](GuildStickerResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pack

> delete_pack(pack_id)
Delete pack

Permanently deletes a pack owned by the authenticated user along with all emojis or stickers contained within it. This action cannot be undone and will remove all associated assets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pack_emoji

> delete_pack_emoji(pack_id, emoji_id, purge)
Delete pack emoji

Permanently deletes an emoji from the specified pack. Requires both pack ID and emoji ID in the path parameters. Accepts an optional \"purge\" query parameter to control whether associated assets are immediately deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
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


## delete_pack_sticker

> delete_pack_sticker(pack_id, sticker_id, purge)
Delete pack sticker

Permanently deletes a sticker from the specified pack. Requires both pack ID and sticker ID in the path parameters. Accepts an optional \"purge\" query parameter to control whether associated assets are immediately deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
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


## install_pack

> install_pack(pack_id)
Install pack

Installs a pack to the authenticated user's collection, making its emojis or stickers available for use. The pack must be publicly accessible or owned by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pack_emojis

> Vec<models::GuildEmojiWithUserResponse> list_pack_emojis(pack_id)
List pack emojis

Returns a list of all emojis contained within the specified pack, including emoji metadata and creator information. Results include emoji ID, name, image URL, and the user who created each emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

[**Vec<models::GuildEmojiWithUserResponse>**](GuildEmojiWithUserResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pack_stickers

> Vec<models::GuildStickerWithUserResponse> list_pack_stickers(pack_id)
List pack stickers

Returns a list of all stickers contained within the specified pack, including sticker metadata and creator information. Results include sticker ID, name, description, tags, image URL, and the user who created each sticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

[**Vec<models::GuildStickerWithUserResponse>**](GuildStickerWithUserResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_packs

> models::PackDashboardResponse list_user_packs()
List user packs

Returns a dashboard view containing all emoji and sticker packs created by or owned by the authenticated user. This includes pack metadata such as name, description, type, and cover image.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PackDashboardResponse**](PackDashboardResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## uninstall_pack

> uninstall_pack(pack_id)
Uninstall pack

Uninstalls a pack from the authenticated user's collection, removing access to its emojis or stickers. This does not delete the pack itself, only removes it from the user's installed list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pack

> models::PackSummaryResponse update_pack(pack_id, pack_update_request)
Update pack

Updates the metadata for an existing pack owned by the authenticated user. Allowed modifications include name, description, and cover image. Returns the updated pack with all current metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
**pack_update_request** | [**PackUpdateRequest**](PackUpdateRequest.md) |  | [required] |

### Return type

[**models::PackSummaryResponse**](PackSummaryResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pack_emoji

> models::GuildEmojiResponse update_pack_emoji(pack_id, emoji_id, guild_emoji_update_request)
Update pack emoji

Updates the name of an existing emoji within the specified pack. Requires both pack ID and emoji ID in the path parameters. Returns the updated emoji with its new name and all existing metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
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


## update_pack_sticker

> models::GuildStickerResponse update_pack_sticker(pack_id, sticker_id, guild_sticker_update_request)
Update pack sticker

Updates the name, description, or tags of an existing sticker within the specified pack. Requires both pack ID and sticker ID in the path parameters. Returns the updated sticker with its new metadata and all existing fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pack_id** | **String** | The ID of the pack | [required] |
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

