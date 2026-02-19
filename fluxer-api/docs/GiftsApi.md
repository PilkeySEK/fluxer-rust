# \GiftsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_gift_code**](GiftsApi.md#get_gift_code) | **GET** /gifts/{code} | Get gift code
[**redeem_gift_code**](GiftsApi.md#redeem_gift_code) | **POST** /gifts/{code}/redeem | Redeem gift code



## get_gift_code

> models::GiftCodeResponse get_gift_code(code)
Get gift code

Retrieves information about a gift code, including sender details and premium entitlements.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code | [required] |

### Return type

[**models::GiftCodeResponse**](GiftCodeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeem_gift_code

> redeem_gift_code(code)
Redeem gift code

Redeems a gift code for the authenticated user, applying premium benefits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The code | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

