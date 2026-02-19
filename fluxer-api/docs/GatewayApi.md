# \GatewayApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gateway_bot**](GatewayApi.md#get_gateway_bot) | **GET** /gateway/bot | Get gateway information



## get_gateway_bot

> models::GatewayBotResponse get_gateway_bot()
Get gateway information

Retrieves gateway connection information and recommended shard count for establishing WebSocket connections.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GatewayBotResponse**](GatewayBotResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

