# \PremiumApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_subscription**](PremiumApi.md#cancel_subscription) | **POST** /premium/cancel-subscription | Cancel subscription
[**create_customer_portal**](PremiumApi.md#create_customer_portal) | **POST** /premium/customer-portal | Create customer portal
[**get_price_ids**](PremiumApi.md#get_price_ids) | **GET** /premium/price-ids | Get Stripe price IDs
[**reactivate_subscription**](PremiumApi.md#reactivate_subscription) | **POST** /premium/reactivate-subscription | Reactivate subscription
[**rejoin_operator_guild**](PremiumApi.md#rejoin_operator_guild) | **POST** /premium/operator/rejoin | Rejoin operator guild
[**rejoin_visionary_guild**](PremiumApi.md#rejoin_visionary_guild) | **POST** /premium/visionary/rejoin | Rejoin visionary guild



## cancel_subscription

> cancel_subscription()
Cancel subscription

Cancels the authenticated user's premium subscription at the end of the current billing period.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_customer_portal

> models::UrlResponse create_customer_portal()
Create customer portal

Creates a session URL for the authenticated user to manage their Stripe subscription via the customer portal.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UrlResponse**](UrlResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_price_ids

> models::PriceIdsResponse get_price_ids(country_code)
Get Stripe price IDs

Retrieves Stripe price IDs for premium subscriptions based on geographic location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country_code** | Option<**String**> |  |  |

### Return type

[**models::PriceIdsResponse**](PriceIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactivate_subscription

> reactivate_subscription()
Reactivate subscription

Reactivates a previously cancelled premium subscription for the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rejoin_operator_guild

> rejoin_operator_guild()
Rejoin operator guild

Adds the authenticated user back to the operator community guild after premium re-subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rejoin_visionary_guild

> rejoin_visionary_guild()
Rejoin visionary guild

Adds the authenticated user back to the visionary community guild after premium re-subscription.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

