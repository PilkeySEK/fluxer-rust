# \ChannelsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_message**](ChannelsApi.md#acknowledge_message) | **POST** /channels/{channel_id}/messages/{message_id}/ack | Acknowledge a message
[**acknowledge_pins**](ChannelsApi.md#acknowledge_pins) | **POST** /channels/{channel_id}/pins/ack | Acknowledge new pin notifications
[**add_group_dm_recipient**](ChannelsApi.md#add_group_dm_recipient) | **PUT** /channels/{channel_id}/recipients/{user_id} | Add recipient to group DM
[**add_reaction**](ChannelsApi.md#add_reaction) | **PUT** /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me | Add reaction to message
[**bulk_delete_messages**](ChannelsApi.md#bulk_delete_messages) | **POST** /channels/{channel_id}/messages/bulk-delete | Bulk delete messages
[**clear_channel_read_state**](ChannelsApi.md#clear_channel_read_state) | **DELETE** /channels/{channel_id}/messages/ack | Clear channel read state
[**complete_chunked_upload**](ChannelsApi.md#complete_chunked_upload) | **POST** /channels/{channel_id}/chunked-uploads/{upload_id}/complete | Complete a chunked upload
[**create_chunked_upload**](ChannelsApi.md#create_chunked_upload) | **POST** /channels/{channel_id}/chunked-uploads | Initiate a chunked upload session
[**delete_channel**](ChannelsApi.md#delete_channel) | **DELETE** /channels/{channel_id} | Delete a channel
[**delete_channel_permission_overwrite**](ChannelsApi.md#delete_channel_permission_overwrite) | **DELETE** /channels/{channel_id}/permissions/{overwrite_id} | Delete permission overwrite
[**delete_message2**](ChannelsApi.md#delete_message2) | **DELETE** /channels/{channel_id}/messages/{message_id} | Delete a message
[**delete_message_attachment**](ChannelsApi.md#delete_message_attachment) | **DELETE** /channels/{channel_id}/messages/{message_id}/attachments/{attachment_id} | Delete a message attachment
[**edit_message**](ChannelsApi.md#edit_message) | **PATCH** /channels/{channel_id}/messages/{message_id} | Edit a message
[**end_call**](ChannelsApi.md#end_call) | **POST** /channels/{channel_id}/call/end | End call session
[**get_call_eligibility**](ChannelsApi.md#get_call_eligibility) | **GET** /channels/{channel_id}/call | Get call eligibility status
[**get_channel**](ChannelsApi.md#get_channel) | **GET** /channels/{channel_id} | Fetch a channel
[**get_message**](ChannelsApi.md#get_message) | **GET** /channels/{channel_id}/messages/{message_id} | Fetch a message
[**get_stream_preview**](ChannelsApi.md#get_stream_preview) | **GET** /streams/{stream_key}/preview | Get stream preview image
[**indicate_typing**](ChannelsApi.md#indicate_typing) | **POST** /channels/{channel_id}/typing | Indicate typing activity
[**list_messages**](ChannelsApi.md#list_messages) | **GET** /channels/{channel_id}/messages | List messages in a channel
[**list_pinned_messages**](ChannelsApi.md#list_pinned_messages) | **GET** /channels/{channel_id}/messages/pins | List pinned messages
[**list_reaction_users**](ChannelsApi.md#list_reaction_users) | **GET** /channels/{channel_id}/messages/{message_id}/reactions/{emoji} | List users who reacted with emoji
[**list_rtc_regions**](ChannelsApi.md#list_rtc_regions) | **GET** /channels/{channel_id}/rtc-regions | List RTC regions
[**pin_message**](ChannelsApi.md#pin_message) | **PUT** /channels/{channel_id}/pins/{message_id} | Pin a message
[**remove_all_reactions**](ChannelsApi.md#remove_all_reactions) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions | Remove all reactions from message
[**remove_all_reactions_for_emoji**](ChannelsApi.md#remove_all_reactions_for_emoji) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji} | Remove all reactions with emoji
[**remove_group_dm_recipient**](ChannelsApi.md#remove_group_dm_recipient) | **DELETE** /channels/{channel_id}/recipients/{user_id} | Remove recipient from group DM
[**remove_own_reaction**](ChannelsApi.md#remove_own_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me | Remove own reaction from message
[**remove_reaction**](ChannelsApi.md#remove_reaction) | **DELETE** /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{target_id} | Remove reaction from message
[**ring_call_recipients**](ChannelsApi.md#ring_call_recipients) | **POST** /channels/{channel_id}/call/ring | Ring call recipients
[**schedule_message**](ChannelsApi.md#schedule_message) | **POST** /channels/{channel_id}/messages/schedule | Schedule a message to send later
[**send_message**](ChannelsApi.md#send_message) | **POST** /channels/{channel_id}/messages | Send a message
[**set_channel_permission_overwrite**](ChannelsApi.md#set_channel_permission_overwrite) | **PUT** /channels/{channel_id}/permissions/{overwrite_id} | Set permission overwrite for channel
[**stop_ringing_call_recipients**](ChannelsApi.md#stop_ringing_call_recipients) | **POST** /channels/{channel_id}/call/stop-ringing | Stop ringing call recipients
[**unpin_message**](ChannelsApi.md#unpin_message) | **DELETE** /channels/{channel_id}/pins/{message_id} | Unpin a message
[**update_call_region**](ChannelsApi.md#update_call_region) | **PATCH** /channels/{channel_id}/call | Update call region
[**update_channel**](ChannelsApi.md#update_channel) | **PATCH** /channels/{channel_id} | Update channel settings
[**update_stream_region**](ChannelsApi.md#update_stream_region) | **PATCH** /streams/{stream_key}/stream | Update stream region
[**upload_chunk**](ChannelsApi.md#upload_chunk) | **PUT** /channels/{channel_id}/chunked-uploads/{upload_id}/chunks/{chunk_index} | Upload a file chunk
[**upload_stream_preview**](ChannelsApi.md#upload_stream_preview) | **POST** /streams/{stream_key}/preview | Upload stream preview image



## acknowledge_message

> acknowledge_message(channel_id, message_id, message_ack_request)
Acknowledge a message

Marks a message as read and records acknowledgement state. Only available for regular user accounts. Updates mention count if provided. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**message_ack_request** | [**MessageAckRequest**](MessageAckRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## acknowledge_pins

> acknowledge_pins(channel_id)
Acknowledge new pin notifications

Marks all new pin notifications in a channel as acknowledged. Clears the notification badge for pinned messages. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_group_dm_recipient

> add_group_dm_recipient(channel_id, user_id)
Add recipient to group DM

Adds a user to a group direct message channel. The requesting user must be a member of the group DM.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**user_id** | **String** | The ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_reaction

> add_reaction(channel_id, message_id, emoji, session_id)
Add reaction to message

Adds an emoji reaction to a message. Each user can react once with each emoji. Cannot be used from unclaimed accounts outside personal notes. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**emoji** | **String** | The emoji | [required] |
**session_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_delete_messages

> bulk_delete_messages(channel_id, bulk_delete_messages_request)
Bulk delete messages

Deletes multiple messages at once. Requires moderation or admin permissions. Commonly used for message cleanup. Messages from different authors can be deleted together. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**bulk_delete_messages_request** | [**BulkDeleteMessagesRequest**](BulkDeleteMessagesRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_channel_read_state

> clear_channel_read_state(channel_id)
Clear channel read state

Clears all read state and acknowledgement records for a channel, marking all messages as unread. Only available for regular user accounts. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_chunked_upload

> models::CompleteChunkedUploadResponse complete_chunked_upload(channel_id, upload_id, complete_chunked_upload_request)
Complete a chunked upload

Completes a chunked upload session by assembling all uploaded chunks. Requires ETags for all chunks. Returns the upload filename that can be referenced when sending a message with the uploaded file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**upload_id** | **String** | The upload id | [required] |
**complete_chunked_upload_request** | [**CompleteChunkedUploadRequest**](CompleteChunkedUploadRequest.md) |  | [required] |

### Return type

[**models::CompleteChunkedUploadResponse**](CompleteChunkedUploadResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunked_upload

> models::CreateChunkedUploadResponse create_chunked_upload(channel_id, create_chunked_upload_request)
Initiate a chunked upload session

Creates a new chunked upload session for uploading large files. Returns the upload ID, expected chunk size, and total chunk count. The client should then upload each chunk individually and complete the upload when all chunks are uploaded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**create_chunked_upload_request** | [**CreateChunkedUploadRequest**](CreateChunkedUploadRequest.md) |  | [required] |

### Return type

[**models::CreateChunkedUploadResponse**](CreateChunkedUploadResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel

> delete_channel(channel_id, silent)
Delete a channel

Permanently removes a channel and all its content. Only server administrators or the channel owner can delete channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**silent** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel_permission_overwrite

> delete_channel_permission_overwrite(channel_id, overwrite_id)
Delete permission overwrite

Removes a permission override from a role or user in the channel, restoring default permissions. Requires channel management rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**overwrite_id** | **String** | The overwrite id | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message2

> delete_message2(channel_id, message_id)
Delete a message

Deletes a message permanently. Only the message author can delete messages (or admins/moderators with proper permissions). Cannot be undone. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message_attachment

> delete_message_attachment(channel_id, message_id, attachment_id)
Delete a message attachment

Removes a specific attachment from a message while keeping the message intact. Only the message author can remove attachments (or admins/moderators). Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**attachment_id** | **String** | The attachment id | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_message

> models::MessageResponseSchema edit_message(channel_id, message_id)
Edit a message

Updates an existing message. Only the message author can edit messages (or admins with proper permissions). Supports updating content, embeds, and attachments. Returns the updated message object. Maintains original message ID and timestamps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

[**models::MessageResponseSchema**](MessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## end_call

> end_call(channel_id)
End call session

Terminates an active voice call in the channel. Records the call end state for all participants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call_eligibility

> models::CallEligibilityResponse get_call_eligibility(channel_id)
Get call eligibility status

Checks whether a call can be initiated in the channel and if there is an active call. Returns ringable status and silent mode flag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**models::CallEligibilityResponse**](CallEligibilityResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> models::ChannelResponse get_channel(channel_id)
Fetch a channel

Retrieves the channel object including metadata, member list, and settings. Requires the user to be a member of the channel with view permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**models::ChannelResponse**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_message

> models::MessageResponseSchema get_message(channel_id, message_id)
Fetch a message

Retrieves a specific message by ID. User must have permission to view the channel and the message must exist. Returns full message details including content, author, reactions, and attachments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

[**models::MessageResponseSchema**](MessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stream_preview

> get_stream_preview(stream_key)
Get stream preview image

Retrieves the current preview thumbnail for a stream. Returns the image with no-store cache headers to ensure freshness.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_key** | **String** | The stream key | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## indicate_typing

> indicate_typing(channel_id)
Indicate typing activity

Notifies other users in the channel that you are actively typing. Typing indicators typically expire after a short period (usually 10 seconds). Returns 204 No Content. Commonly called repeatedly while the user is composing a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_messages

> Vec<models::MessageResponseSchema> list_messages(channel_id, limit, before, after, around)
List messages in a channel

Retrieves a paginated list of messages from a channel. User must have permission to view the channel. Supports pagination via limit, before, after, and around parameters. Returns messages in reverse chronological order (newest first).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**limit** | Option<**String**> |  |  |
**before** | Option<**String**> |  |  |
**after** | Option<**String**> |  |  |
**around** | Option<**String**> |  |  |

### Return type

[**Vec<models::MessageResponseSchema>**](MessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pinned_messages

> models::ChannelPinsResponse list_pinned_messages(channel_id, limit, before)
List pinned messages

Retrieves a paginated list of messages pinned in a channel. User must have permission to view the channel. Supports pagination via limit and before parameters. Returns pinned messages with their pin timestamps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**limit** | Option<**i32**> |  |  |
**before** | Option<**String**> |  |  |

### Return type

[**models::ChannelPinsResponse**](ChannelPinsResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_reaction_users

> Vec<models::UserPartialResponse> list_reaction_users(channel_id, message_id, emoji, limit, after)
List users who reacted with emoji

Retrieves a paginated list of users who reacted to a message with a specific emoji. Supports pagination via limit and after parameters. Returns user objects for each reaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**emoji** | **String** | The emoji | [required] |
**limit** | Option<**i32**> |  |  |
**after** | Option<**String**> |  |  |

### Return type

[**Vec<models::UserPartialResponse>**](UserPartialResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_rtc_regions

> Vec<models::RtcRegionResponse> list_rtc_regions(channel_id)
List RTC regions

Returns available voice and video calling regions for the channel, used to optimise connection quality. Requires membership with call permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**Vec<models::RtcRegionResponse>**](RtcRegionResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_message

> pin_message(channel_id, message_id)
Pin a message

Pins a message to the channel. Requires permission to manage pins (typically moderator or higher). Pinned messages are highlighted and searchable. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_all_reactions

> remove_all_reactions(channel_id, message_id)
Remove all reactions from message

Removes all emoji reactions from a message, regardless of emoji type or user. All reactions are permanently deleted. Requires moderator or higher permissions. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_all_reactions_for_emoji

> remove_all_reactions_for_emoji(channel_id, message_id, emoji)
Remove all reactions with emoji

Removes all emoji reactions of a specific type from a message. All users' reactions with that emoji are deleted. Requires moderator or higher permissions. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**emoji** | **String** | The emoji | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_dm_recipient

> remove_group_dm_recipient(channel_id, user_id, silent)
Remove recipient from group DM

Removes a user from a group direct message channel. The requesting user must be a member with appropriate permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**user_id** | **String** | The ID of the user | [required] |
**silent** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_own_reaction

> remove_own_reaction(channel_id, message_id, emoji, session_id)
Remove own reaction from message

Removes your own emoji reaction from a message. Returns 204 No Content on success. Has no effect if you haven't reacted with that emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**emoji** | **String** | The emoji | [required] |
**session_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_reaction

> remove_reaction(channel_id, message_id, emoji, target_id, session_id)
Remove reaction from message

Removes a specific user's emoji reaction from a message. Requires moderator or higher permissions to remove reactions from other users. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**emoji** | **String** | The emoji | [required] |
**target_id** | **String** | The target id | [required] |
**session_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ring_call_recipients

> ring_call_recipients(channel_id, call_ring_body_schema)
Ring call recipients

Sends ringing notifications to specified users in a call. If no recipients are specified, rings all channel members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**call_ring_body_schema** | [**CallRingBodySchema**](CallRingBodySchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_message

> models::ScheduledMessageResponseSchema schedule_message(channel_id)
Schedule a message to send later

Schedules a message to be sent at a specified time. Only available for regular user accounts. Requires permission to send messages in the target channel. Message is sent automatically at the scheduled time. Returns the scheduled message object with delivery time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**models::ScheduledMessageResponseSchema**](ScheduledMessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message

> models::MessageResponseSchema send_message(channel_id)
Send a message

Sends a new message to a channel. Requires permission to send messages in the target channel. Supports text content, embeds, attachments (multipart), and mentions. Returns the created message object with full details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**models::MessageResponseSchema**](MessageResponseSchema.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_channel_permission_overwrite

> set_channel_permission_overwrite(channel_id, overwrite_id, permission_overwrite_create_request)
Set permission overwrite for channel

Creates or updates permission overrides for a role or user in the channel. Allows fine-grained control over who can view, send messages, or manage the channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**overwrite_id** | **String** | The overwrite id | [required] |
**permission_overwrite_create_request** | [**PermissionOverwriteCreateRequest**](PermissionOverwriteCreateRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_ringing_call_recipients

> stop_ringing_call_recipients(channel_id, call_ring_body_schema)
Stop ringing call recipients

Stops ringing notifications for specified users in a call. Allows callers to stop notifying users who have declined or not responded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**call_ring_body_schema** | [**CallRingBodySchema**](CallRingBodySchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpin_message

> unpin_message(channel_id, message_id)
Unpin a message

Unpins a message from the channel. Requires permission to manage pins. The message remains in the channel but is no longer highlighted. Returns 204 No Content on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_call_region

> update_call_region(channel_id, call_update_body_schema)
Update call region

Changes the voice server region for an active call to optimise latency and connection quality.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**call_update_body_schema** | [**CallUpdateBodySchema**](CallUpdateBodySchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel

> models::ChannelResponse update_channel(channel_id, channel_update_request)
Update channel settings

Modifies channel properties such as name, description, topic, nsfw flag, and slowmode. Requires management permissions in the channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**channel_update_request** | [**ChannelUpdateRequest**](ChannelUpdateRequest.md) |  | [required] |

### Return type

[**models::ChannelResponse**](ChannelResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stream_region

> update_stream_region(stream_key, stream_update_body_schema)
Update stream region

Changes the media server region for an active stream. Used to optimise bandwidth and latency for streaming.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_key** | **String** | The stream key | [required] |
**stream_update_body_schema** | [**StreamUpdateBodySchema**](StreamUpdateBodySchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_chunk

> models::UploadChunkResponse upload_chunk(channel_id, upload_id, chunk_index)
Upload a file chunk

Uploads a single chunk of a file as part of a chunked upload session. The chunk index is zero-based. Returns an ETag that must be provided when completing the upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**upload_id** | **String** | The upload id | [required] |
**chunk_index** | **String** | The chunk index | [required] |

### Return type

[**models::UploadChunkResponse**](UploadChunkResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_stream_preview

> upload_stream_preview(stream_key, stream_preview_upload_body_schema)
Upload stream preview image

Uploads a custom thumbnail image for the stream. The image is scanned for content policy violations and stored securely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stream_key** | **String** | The stream key | [required] |
**stream_preview_upload_body_schema** | [**StreamPreviewUploadBodySchema**](StreamPreviewUploadBodySchema.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

