# \ReadStatesApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ack_bulk_messages**](ReadStatesApi.md#ack_bulk_messages) | **POST** /read-states/ack-bulk | Mark channels as read



## ack_bulk_messages

> ack_bulk_messages(read_state_ack_bulk_request)
Mark channels as read

Marks multiple channels as read for the authenticated user in bulk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**read_state_ack_bulk_request** | [**ReadStateAckBulkRequest**](ReadStateAckBulkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

