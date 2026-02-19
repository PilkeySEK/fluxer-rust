# \SearchApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_messages**](SearchApi.md#search_messages) | **POST** /search/messages | Search messages



## search_messages

> models::MessageSearchResponse search_messages(global_search_messages_request)
Search messages

Searches for messages across guilds and channels accessible to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**global_search_messages_request** | [**GlobalSearchMessagesRequest**](GlobalSearchMessagesRequest.md) |  | [required] |

### Return type

[**models::MessageSearchResponse**](MessageSearchResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

