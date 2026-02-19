# \DiscoveryApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_for_discovery**](DiscoveryApi.md#apply_for_discovery) | **POST** /guilds/{guild_id}/discovery | Apply for guild discovery
[**edit_discovery_application**](DiscoveryApi.md#edit_discovery_application) | **PATCH** /guilds/{guild_id}/discovery | Edit discovery application
[**get_discovery_status**](DiscoveryApi.md#get_discovery_status) | **GET** /guilds/{guild_id}/discovery | Get discovery status
[**join_discovery_guild**](DiscoveryApi.md#join_discovery_guild) | **POST** /discovery/guilds/{guild_id}/join | Join a discoverable guild
[**list_discovery_categories**](DiscoveryApi.md#list_discovery_categories) | **GET** /discovery/categories | List discovery categories
[**search_discovery_guilds**](DiscoveryApi.md#search_discovery_guilds) | **GET** /discovery/guilds | Search discoverable guilds
[**withdraw_discovery_application**](DiscoveryApi.md#withdraw_discovery_application) | **DELETE** /guilds/{guild_id}/discovery | Withdraw discovery application



## apply_for_discovery

> models::DiscoveryApplicationResponse apply_for_discovery(guild_id, discovery_application_request)
Apply for guild discovery

Submit a discovery application for a guild. Requires MANAGE_GUILD permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**discovery_application_request** | [**DiscoveryApplicationRequest**](DiscoveryApplicationRequest.md) |  | [required] |

### Return type

[**models::DiscoveryApplicationResponse**](DiscoveryApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_discovery_application

> models::DiscoveryApplicationResponse edit_discovery_application(guild_id, discovery_application_patch_request)
Edit discovery application

Update the description or category of an existing discovery application. Requires MANAGE_GUILD permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |
**discovery_application_patch_request** | [**DiscoveryApplicationPatchRequest**](DiscoveryApplicationPatchRequest.md) |  | [required] |

### Return type

[**models::DiscoveryApplicationResponse**](DiscoveryApplicationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_discovery_status

> models::DiscoveryStatusResponse get_discovery_status(guild_id)
Get discovery status

Get the current discovery status and eligibility of a guild. Requires MANAGE_GUILD permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

[**models::DiscoveryStatusResponse**](DiscoveryStatusResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## join_discovery_guild

> join_discovery_guild(guild_id)
Join a discoverable guild

Join a guild that is listed in discovery without needing an invite.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_discovery_categories

> Vec<models::DiscoveryCategoryResponse> list_discovery_categories()
List discovery categories

Returns the list of available discovery categories.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DiscoveryCategoryResponse>**](DiscoveryCategoryResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_discovery_guilds

> models::DiscoveryGuildListResponse search_discovery_guilds(query, category, sort_by, limit, offset)
Search discoverable guilds

Search for guilds listed in the discovery directory.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |
**category** | Option<**i32**> |  |  |
**sort_by** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**offset** | Option<**i64**> |  |  |

### Return type

[**models::DiscoveryGuildListResponse**](DiscoveryGuildListResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_discovery_application

> withdraw_discovery_application(guild_id)
Withdraw discovery application

Withdraw a discovery application or remove a guild from discovery. Requires MANAGE_GUILD permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**guild_id** | **String** | The ID of the guild | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

