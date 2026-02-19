# \ThemesApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_theme**](ThemesApi.md#create_theme) | **POST** /users/@me/themes | Create theme



## create_theme

> models::ThemeCreateResponse create_theme(theme_create_request)
Create theme

Creates a new custom theme with CSS styling that can be shared with other users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme_create_request** | [**ThemeCreateRequest**](ThemeCreateRequest.md) |  | [required] |

### Return type

[**models::ThemeCreateResponse**](ThemeCreateResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

