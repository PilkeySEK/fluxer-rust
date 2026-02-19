# \WebhooksApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook**](WebhooksApi.md#create_webhook) | **POST** /channels/{channel_id}/webhooks | Create webhook
[**delete_webhook**](WebhooksApi.md#delete_webhook) | **DELETE** /webhooks/{webhook_id} | Delete webhook
[**delete_webhook_with_token**](WebhooksApi.md#delete_webhook_with_token) | **DELETE** /webhooks/{webhook_id}/{token} | Delete webhook with token
[**execute_github_webhook**](WebhooksApi.md#execute_github_webhook) | **POST** /webhooks/{webhook_id}/{token}/github | Execute GitHub webhook
[**execute_sentry_webhook**](WebhooksApi.md#execute_sentry_webhook) | **POST** /webhooks/{webhook_id}/{token}/sentry | Execute Sentry webhook
[**execute_slack_webhook**](WebhooksApi.md#execute_slack_webhook) | **POST** /webhooks/{webhook_id}/{token}/slack | Execute Slack webhook
[**execute_webhook**](WebhooksApi.md#execute_webhook) | **POST** /webhooks/{webhook_id}/{token} | Execute webhook
[**get_webhook**](WebhooksApi.md#get_webhook) | **GET** /webhooks/{webhook_id} | Get webhook
[**get_webhook_with_token**](WebhooksApi.md#get_webhook_with_token) | **GET** /webhooks/{webhook_id}/{token} | Get webhook with token
[**list_channel_webhooks**](WebhooksApi.md#list_channel_webhooks) | **GET** /channels/{channel_id}/webhooks | List channel webhooks
[**list_guild_webhooks**](WebhooksApi.md#list_guild_webhooks) | **GET** /guilds/{guild_id}/webhooks | List guild webhooks
[**update_webhook**](WebhooksApi.md#update_webhook) | **PATCH** /webhooks/{webhook_id} | Update webhook
[**update_webhook_with_token**](WebhooksApi.md#update_webhook_with_token) | **PATCH** /webhooks/{webhook_id}/{token} | Update webhook with token



## create_webhook

> models::WebhookResponse create_webhook(channel_id, webhook_create_request)
Create webhook

Creates a new webhook in the specified channel with the provided name and optional avatar. Returns the newly created webhook object including its ID and token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**webhook_create_request** | [**WebhookCreateRequest**](WebhookCreateRequest.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook

> delete_webhook(webhook_id)
Delete webhook

Permanently deletes the specified webhook. This action cannot be undone. Returns a 204 status code on successful deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhook_with_token

> delete_webhook_with_token(webhook_id, token)
Delete webhook with token

Permanently deletes the specified webhook using its ID and token for authentication. This action cannot be undone. Returns a 204 status code on successful deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_github_webhook

> execute_github_webhook(webhook_id, token, git_hub_webhook)
Execute GitHub webhook

Receives and processes GitHub webhook events, formatting them as messages in the configured channel. Reads event type from X-GitHub-Event header and delivery ID from X-GitHub-Delivery header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |
**git_hub_webhook** | [**GitHubWebhook**](GitHubWebhook.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_sentry_webhook

> execute_sentry_webhook(webhook_id, token, sentry_webhook)
Execute Sentry webhook

Receives and processes Sentry error tracking webhook events, formatting them as messages in the configured channel. Reads event type from X-Sentry-Event header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |
**sentry_webhook** | [**SentryWebhook**](SentryWebhook.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_slack_webhook

> String execute_slack_webhook(webhook_id, token, slack_webhook_request)
Execute Slack webhook

Receives and processes Slack-formatted webhook payloads, converting them to messages in the configured channel. Returns \"ok\" as plain text with a 200 status code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |
**slack_webhook_request** | [**SlackWebhookRequest**](SlackWebhookRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## execute_webhook

> models::MessageResponseSchema execute_webhook(webhook_id, token, wait)
Execute webhook

Executes the webhook by sending a message to its configured channel. If the wait query parameter is true, returns the created message object; otherwise returns a 204 status with no content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |
**wait** | Option<**String**> |  |  |

### Return type

[**models::MessageResponseSchema**](MessageResponseSchema.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook

> models::WebhookResponse get_webhook(webhook_id)
Get webhook

Retrieves detailed information about a specific webhook by its ID. Requires authentication and appropriate permissions to access the webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhook_with_token

> models::WebhookTokenResponse get_webhook_with_token(webhook_id, token)
Get webhook with token

Retrieves detailed information about a specific webhook using its ID and token. No authentication required as the token serves as the credential. Returns the webhook object without creator user data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |

### Return type

[**models::WebhookTokenResponse**](WebhookTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channel_webhooks

> Vec<models::WebhookResponse> list_channel_webhooks(channel_id)
List channel webhooks

Returns a list of all webhooks configured in the specified channel. Requires the user to have appropriate permissions to view webhooks in the channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |

### Return type

[**Vec<models::WebhookResponse>**](WebhookResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_guild_webhooks

> Vec<models::WebhookResponse> list_guild_webhooks(guild_id)
List guild webhooks

Returns a list of all webhooks configured in the specified guild. Requires the user to have appropriate permissions to view webhooks in the guild.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**Vec<models::WebhookResponse>**](WebhookResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook

> models::WebhookResponse update_webhook(webhook_id, webhook_update_request)
Update webhook

Updates the specified webhook with new settings such as name, avatar, or target channel. All fields are optional. Returns the updated webhook object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**webhook_update_request** | [**WebhookUpdateRequest**](WebhookUpdateRequest.md) |  | [required] |

### Return type

[**models::WebhookResponse**](WebhookResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_webhook_with_token

> models::WebhookTokenResponse update_webhook_with_token(webhook_id, token, webhook_token_update_request)
Update webhook with token

Updates the specified webhook using its ID and token for authentication. Allows modification of name or avatar. Returns the updated webhook object without creator user data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** | The ID of the webhook | [required] |
**token** | **String** | The token | [required] |
**webhook_token_update_request** | [**WebhookTokenUpdateRequest**](WebhookTokenUpdateRequest.md) |  | [required] |

### Return type

[**models::WebhookTokenResponse**](WebhookTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

