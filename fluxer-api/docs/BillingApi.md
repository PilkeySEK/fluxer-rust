# \BillingApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_checkout_session**](BillingApi.md#create_checkout_session) | **POST** /stripe/checkout/subscription | Create checkout session
[**create_gift_checkout_session**](BillingApi.md#create_gift_checkout_session) | **POST** /stripe/checkout/gift | Create gift checkout session
[**process_stripe_webhook**](BillingApi.md#process_stripe_webhook) | **POST** /stripe/webhook | Process Stripe webhook



## create_checkout_session

> models::UrlResponse create_checkout_session(create_checkout_session_request)
Create checkout session

Initiates a Stripe checkout session for user subscription purchases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_checkout_session_request** | [**CreateCheckoutSessionRequest**](CreateCheckoutSessionRequest.md) |  | [required] |

### Return type

[**models::UrlResponse**](UrlResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_gift_checkout_session

> models::UrlResponse create_gift_checkout_session(create_checkout_session_request)
Create gift checkout session

Creates a checkout session for purchasing premium gifts to send to other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_checkout_session_request** | [**CreateCheckoutSessionRequest**](CreateCheckoutSessionRequest.md) |  | [required] |

### Return type

[**models::UrlResponse**](UrlResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## process_stripe_webhook

> models::WebhookReceivedResponse process_stripe_webhook()
Process Stripe webhook

Handles incoming Stripe webhook events for payment processing and subscription management.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookReceivedResponse**](WebhookReceivedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

