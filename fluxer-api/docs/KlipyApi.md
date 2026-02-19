# \KlipyApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_klipy_featured**](KlipyApi.md#get_klipy_featured) | **GET** /klipy/featured | Get featured KLIPY GIFs
[**get_klipy_search_suggestions**](KlipyApi.md#get_klipy_search_suggestions) | **GET** /klipy/suggest | Get KLIPY search suggestions
[**get_klipy_trending_gifs**](KlipyApi.md#get_klipy_trending_gifs) | **GET** /klipy/trending-gifs | Get trending KLIPY GIFs
[**register_klipy_gif_share**](KlipyApi.md#register_klipy_gif_share) | **POST** /klipy/register-share | Register KLIPY GIF share
[**search_klipy_gifs**](KlipyApi.md#search_klipy_gifs) | **GET** /klipy/search | Search KLIPY GIFs



## get_klipy_featured

> models::KlipyFeaturedResponse get_klipy_featured(locale)
Get featured KLIPY GIFs

Retrieves currently featured GIFs from KLIPY based on user locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**models::KlipyFeaturedResponse**](KlipyFeaturedResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_klipy_search_suggestions

> Vec<String> get_klipy_search_suggestions(q, locale)
Get KLIPY search suggestions

Returns search term suggestions from KLIPY based on the partial query provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

**Vec<String>**

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_klipy_trending_gifs

> Vec<models::KlipyGifResponse> get_klipy_trending_gifs(locale)
Get trending KLIPY GIFs

Retrieves trending GIFs from KLIPY based on user locale and popularity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**Vec<models::KlipyGifResponse>**](KlipyGifResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_klipy_gif_share

> register_klipy_gif_share(klipy_register_share_request)
Register KLIPY GIF share

Registers a shared GIF with KLIPY to track usage and analytics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**klipy_register_share_request** | [**KlipyRegisterShareRequest**](KlipyRegisterShareRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_klipy_gifs

> Vec<models::KlipyGifResponse> search_klipy_gifs(q, locale)
Search KLIPY GIFs

Searches KLIPY for GIFs matching the given query string and locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**Vec<models::KlipyGifResponse>**](KlipyGifResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

