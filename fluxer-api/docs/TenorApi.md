# \TenorApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_tenor_featured**](TenorApi.md#get_tenor_featured) | **GET** /tenor/featured | Get featured Tenor GIFs
[**get_tenor_search_suggestions**](TenorApi.md#get_tenor_search_suggestions) | **GET** /tenor/suggest | Get Tenor search suggestions
[**get_tenor_trending_gifs**](TenorApi.md#get_tenor_trending_gifs) | **GET** /tenor/trending-gifs | Get trending Tenor GIFs
[**register_tenor_gif_share**](TenorApi.md#register_tenor_gif_share) | **POST** /tenor/register-share | Register Tenor GIF share
[**search_tenor_gifs**](TenorApi.md#search_tenor_gifs) | **GET** /tenor/search | Search Tenor GIFs



## get_tenor_featured

> models::TenorFeaturedResponse get_tenor_featured(locale)
Get featured Tenor GIFs

Retrieves currently featured GIFs from Tenor based on user locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**models::TenorFeaturedResponse**](TenorFeaturedResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenor_search_suggestions

> Vec<String> get_tenor_search_suggestions(q, locale)
Get Tenor search suggestions

Returns search term suggestions from Tenor based on the partial query provided.

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


## get_tenor_trending_gifs

> Vec<models::TenorGifResponse> get_tenor_trending_gifs(locale)
Get trending Tenor GIFs

Retrieves trending/featured GIFs from Tenor based on user locale and popularity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**Vec<models::TenorGifResponse>**](TenorGifResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_tenor_gif_share

> register_tenor_gif_share(tenor_register_share_request)
Register Tenor GIF share

Registers a shared GIF with Tenor to help tune search results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenor_register_share_request** | [**TenorRegisterShareRequest**](TenorRegisterShareRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_tenor_gifs

> Vec<models::TenorGifResponse> search_tenor_gifs(q, locale)
Search Tenor GIFs

Searches Tenor for GIFs matching the given query string and locale.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | **String** |  | [required] |
**locale** | Option<[**Locale**](Locale.md)> |  |  |

### Return type

[**Vec<models::TenorGifResponse>**](TenorGifResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

