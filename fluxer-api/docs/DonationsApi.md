# \DonationsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_donation_checkout**](DonationsApi.md#create_donation_checkout) | **POST** /donations/checkout | Create donation checkout session
[**manage_donation**](DonationsApi.md#manage_donation) | **GET** /donations/manage | Manage donation subscription
[**request_donation_magic_link**](DonationsApi.md#request_donation_magic_link) | **POST** /donations/request-link | Request donation management link



## create_donation_checkout

> models::DonationCheckoutResponse create_donation_checkout(donation_checkout_request)
Create donation checkout session

Creates a Stripe checkout session for a recurring donation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**donation_checkout_request** | [**DonationCheckoutRequest**](DonationCheckoutRequest.md) |  | [required] |

### Return type

[**models::DonationCheckoutResponse**](DonationCheckoutResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_donation

> manage_donation(token)
Manage donation subscription

Validates the magic link token and redirects to Stripe billing portal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_donation_magic_link

> request_donation_magic_link(donation_request_link_request)
Request donation management link

Sends a magic link email to the provided address for managing recurring donations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**donation_request_link_request** | [**DonationRequestLinkRequest**](DonationRequestLinkRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

