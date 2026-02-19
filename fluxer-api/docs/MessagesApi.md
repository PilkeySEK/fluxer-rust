# \MessagesApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acknowledge_message**](MessagesApi.md#acknowledge_message) | **POST** /channels/{channel_id}/messages/{message_id}/ack | Acknowledge a message
[**bulk_delete_messages**](MessagesApi.md#bulk_delete_messages) | **POST** /channels/{channel_id}/messages/bulk-delete | Bulk delete messages
[**clear_channel_read_state**](MessagesApi.md#clear_channel_read_state) | **DELETE** /channels/{channel_id}/messages/ack | Clear channel read state
[**delete_message2**](MessagesApi.md#delete_message2) | **DELETE** /channels/{channel_id}/messages/{message_id} | Delete a message
[**delete_message_attachment**](MessagesApi.md#delete_message_attachment) | **DELETE** /channels/{channel_id}/messages/{message_id}/attachments/{attachment_id} | Delete a message attachment
[**edit_message**](MessagesApi.md#edit_message) | **PATCH** /channels/{channel_id}/messages/{message_id} | Edit a message
[**get_message**](MessagesApi.md#get_message) | **GET** /channels/{channel_id}/messages/{message_id} | Fetch a message
[**indicate_typing**](MessagesApi.md#indicate_typing) | **POST** /channels/{channel_id}/typing | Indicate typing activity
[**list_messages**](MessagesApi.md#list_messages) | **GET** /channels/{channel_id}/messages | List messages in a channel
[**send_message**](MessagesApi.md#send_message) | **POST** /channels/{channel_id}/messages | Send a message



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

