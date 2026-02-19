# \SavedMediaApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_meme_from_message**](SavedMediaApi.md#create_meme_from_message) | **POST** /channels/{channel_id}/messages/{message_id}/memes | Create meme from message
[**create_meme_from_url**](SavedMediaApi.md#create_meme_from_url) | **POST** /users/@me/memes | Create meme from URL
[**delete_favorite_meme**](SavedMediaApi.md#delete_favorite_meme) | **DELETE** /users/@me/memes/{meme_id} | Delete favorite meme
[**get_favorite_meme**](SavedMediaApi.md#get_favorite_meme) | **GET** /users/@me/memes/{meme_id} | Get favorite meme
[**list_favorite_memes**](SavedMediaApi.md#list_favorite_memes) | **GET** /users/@me/memes | List favorite memes
[**update_favorite_meme**](SavedMediaApi.md#update_favorite_meme) | **PATCH** /users/@me/memes/{meme_id} | Update favorite meme



## create_meme_from_message

> models::FavoriteMemeResponse create_meme_from_message(channel_id, message_id, create_favorite_meme_body_schema)
Create meme from message

Saves a message attachment as a favorite meme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | The ID of the channel | [required] |
**message_id** | **String** | The ID of the message | [required] |
**create_favorite_meme_body_schema** | [**CreateFavoriteMemeBodySchema**](CreateFavoriteMemeBodySchema.md) |  | [required] |

### Return type

[**models::FavoriteMemeResponse**](FavoriteMemeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_meme_from_url

> models::FavoriteMemeResponse create_meme_from_url(create_favorite_meme_from_url_body_schema)
Create meme from URL

Saves a new meme to favorites from a provided URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_favorite_meme_from_url_body_schema** | [**CreateFavoriteMemeFromUrlBodySchema**](CreateFavoriteMemeFromUrlBodySchema.md) |  | [required] |

### Return type

[**models::FavoriteMemeResponse**](FavoriteMemeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_favorite_meme

> delete_favorite_meme(meme_id)
Delete favorite meme

Removes a favorite meme from the authenticated user's collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meme_id** | **String** | The meme id | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_favorite_meme

> models::FavoriteMemeResponse get_favorite_meme(meme_id)
Get favorite meme

Retrieves a specific favorite meme by ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meme_id** | **String** | The meme id | [required] |

### Return type

[**models::FavoriteMemeResponse**](FavoriteMemeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_favorite_memes

> Vec<models::FavoriteMemeResponse> list_favorite_memes()
List favorite memes

Retrieves all memes saved as favorites by the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::FavoriteMemeResponse>**](FavoriteMemeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_favorite_meme

> models::FavoriteMemeResponse update_favorite_meme(meme_id, update_favorite_meme_body_schema)
Update favorite meme

Updates details of a favorite meme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**meme_id** | **String** | The meme id | [required] |
**update_favorite_meme_body_schema** | [**UpdateFavoriteMemeBodySchema**](UpdateFavoriteMemeBodySchema.md) |  | [required] |

### Return type

[**models::FavoriteMemeResponse**](FavoriteMemeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

