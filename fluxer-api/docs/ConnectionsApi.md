# \ConnectionsApi

All URIs are relative to *https://api.fluxer.app/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize_bluesky_connection**](ConnectionsApi.md#authorize_bluesky_connection) | **POST** /users/@me/connections/bluesky/authorize | Start Bluesky OAuth flow
[**delete_connection**](ConnectionsApi.md#delete_connection) | **DELETE** /users/@me/connections/{type}/{connection_id} | Delete connection
[**initiate_connection**](ConnectionsApi.md#initiate_connection) | **POST** /users/@me/connections | Initiate connection
[**list_connections**](ConnectionsApi.md#list_connections) | **GET** /users/@me/connections | List user connections
[**reorder_connections**](ConnectionsApi.md#reorder_connections) | **PATCH** /users/@me/connections/reorder | Reorder connections
[**update_connection**](ConnectionsApi.md#update_connection) | **PATCH** /users/@me/connections/{type}/{connection_id} | Update connection
[**verify_and_create_connection**](ConnectionsApi.md#verify_and_create_connection) | **POST** /users/@me/connections/verify | Verify and create connection
[**verify_connection**](ConnectionsApi.md#verify_connection) | **POST** /users/@me/connections/{type}/{connection_id}/verify | Verify connection



## authorize_bluesky_connection

> models::BlueskyAuthorizeResponse authorize_bluesky_connection(bluesky_authorize_request)
Start Bluesky OAuth flow

Initiates the Bluesky OAuth2 authorisation flow and returns a URL to redirect the user to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bluesky_authorize_request** | [**BlueskyAuthorizeRequest**](BlueskyAuthorizeRequest.md) |  | [required] |

### Return type

[**models::BlueskyAuthorizeResponse**](BlueskyAuthorizeResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connection

> delete_connection(r#type, connection_id)
Delete connection

Removes an external service connection from the authenticated user's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type | [required] |
**connection_id** | **String** | The connection id | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initiate_connection

> models::ConnectionVerificationResponse initiate_connection(create_connection_request)
Initiate connection

Initiates a new external service connection and returns verification instructions. No database record is created until verification succeeds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_connection_request** | [**CreateConnectionRequest**](CreateConnectionRequest.md) |  | [required] |

### Return type

[**models::ConnectionVerificationResponse**](ConnectionVerificationResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connections

> Vec<models::ConnectionResponse> list_connections()
List user connections

Retrieves all external service connections for the authenticated user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ConnectionResponse>**](ConnectionResponse.md)

### Authorization

[oauth2Token](../README.md#oauth2Token), [sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reorder_connections

> reorder_connections(reorder_connections_request)
Reorder connections

Updates the display order of multiple connections in a single operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reorder_connections_request** | [**ReorderConnectionsRequest**](ReorderConnectionsRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connection

> update_connection(r#type, connection_id, update_connection_request)
Update connection

Updates visibility and sort order settings for an external service connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type | [required] |
**connection_id** | **String** | The connection id | [required] |
**update_connection_request** | [**UpdateConnectionRequest**](UpdateConnectionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_and_create_connection

> models::ConnectionResponse verify_and_create_connection(verify_and_create_connection_request)
Verify and create connection

Verifies the external service connection using the initiation token and creates the connection record on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_and_create_connection_request** | [**VerifyAndCreateConnectionRequest**](VerifyAndCreateConnectionRequest.md) |  | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_connection

> models::ConnectionResponse verify_connection(r#type, connection_id)
Verify connection

Triggers verification for an external service connection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** | The type | [required] |
**connection_id** | **String** | The connection id | [required] |

### Return type

[**models::ConnectionResponse**](ConnectionResponse.md)

### Authorization

[sessionToken](../README.md#sessionToken), [botToken](../README.md#botToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

